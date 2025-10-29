// IndexedDB Helper
class LycorisDB {
    private db: IDBDatabase | null = null;
    private readonly dbName = 'LycorisDB';
    private readonly storeName = 'dictionary';
    private readonly version = 1;

    async init(): Promise<void> {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(this.dbName, this.version);
            
            request.onerror = () => reject(request.error);
            request.onsuccess = () => {
                this.db = request.result;
                resolve();
            };
            
            request.onupgradeneeded = (event) => {
                const db = (event.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains(this.storeName)) {
                    db.createObjectStore(this.storeName);
                }
            };
        });
    }

    async save(key: string, value: any): Promise<void> {
        if (!this.db) throw new Error('Database not initialized');
        
        return new Promise((resolve, reject) => {
            const transaction = this.db!.transaction([this.storeName], 'readwrite');
            const store = transaction.objectStore(this.storeName);
            const request = store.put(value, key);
            
            request.onerror = () => reject(request.error);
            request.onsuccess = () => resolve();
        });
    }

    async load(key: string): Promise<any> {
        if (!this.db) throw new Error('Database not initialized');
        
        return new Promise((resolve, reject) => {
            const transaction = this.db!.transaction([this.storeName], 'readonly');
            const store = transaction.objectStore(this.storeName);
            const request = store.get(key);
            
            request.onerror = () => reject(request.error);
            request.onsuccess = () => resolve(request.result);
        });
    }
}

// Web Worker Manager
class WorkerManager {
    private worker: Worker | null = null;
    private messageId = 0;
    private pendingMessages: Map<number, {resolve: Function, reject: Function}> = new Map();
    private ready = false;

    async init(): Promise<void> {
        return new Promise((resolve, reject) => {
            this.worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });
            
            this.worker.onmessage = (event) => {
                const data = event.data;
                
                if (data.type === 'ready') {
                    this.ready = true;
                    this.sendMessage('init', null).then(resolve).catch(reject);
                    return;
                }
                
                if (data.id !== undefined && this.pendingMessages.has(data.id)) {
                    const { resolve, reject } = this.pendingMessages.get(data.id)!;
                    this.pendingMessages.delete(data.id);
                    
                    if (data.type === 'success') {
                        resolve(data.data);
                    } else {
                        reject(new Error(data.data));
                    }
                }
            };
            
            this.worker.onerror = (error) => {
                console.error('Worker error:', error);
                reject(error);
            };
        });
    }

    private sendMessage(type: string, data: any): Promise<any> {
        return new Promise((resolve, reject) => {
            if (!this.worker || !this.ready) {
                reject(new Error('Worker not ready'));
                return;
            }
            
            const id = this.messageId++;
            this.pendingMessages.set(id, { resolve, reject });
            
            this.worker.postMessage({ type, data, id });
            
            // タイムアウト設定（超巨大数の計算用に長めに設定）
            setTimeout(() => {
                if (this.pendingMessages.has(id)) {
                    this.pendingMessages.delete(id);
                    reject(new Error('Operation timeout'));
                }
            }, 60000); // 60秒タイムアウト
        });
    }

    async execute(code: string): Promise<any> {
        return this.sendMessage('execute', code);
    }

    async getState(): Promise<any> {
        return this.sendMessage('getState', null);
    }

    async loadState(state: string): Promise<any> {
        return this.sendMessage('loadState', state);
    }

    terminate() {
        if (this.worker) {
            this.worker.terminate();
            this.worker = null;
            this.ready = false;
        }
    }
}

// Main UI Class
export class LycorisUI {
    private workerManager: WorkerManager;
    private db: LycorisDB;
    private isMobile: boolean = false;
    private currentView: 'editor' | 'output' = 'editor';
    private isProcessing: boolean = false;
    private currentState: any = null;

    constructor() {
        this.workerManager = new WorkerManager();
        this.db = new LycorisDB();
        this.isMobile = window.innerWidth <= 768;
        window.addEventListener('resize', () => {
            this.isMobile = window.innerWidth <= 768;
            this.updateLayout();
        });
    }

    async init() {
        try {
            // Show loading message
            this.showLoading('Initializing Lycoris...');
            
            // Initialize IndexedDB
            await this.db.init();
            
            // Initialize Web Worker
            await this.workerManager.init();
            
            // Load saved state
            await this.loadSavedState();
            
            // Setup UI
            this.setupDOM();
            this.attachEventListeners();
            this.updateDisplay();
            
        } catch (error) {
            console.error('Initialization error:', error);
            this.showError('Failed to initialize: ' + error);
        }
    }

    private showLoading(message: string) {
        const app = document.getElementById('app');
        if (app) {
            app.innerHTML = `<div class="loading">${message}</div>`;
        }
    }

    private setupDOM() {
        const app = document.getElementById('app');
        if (!app) return;

        app.innerHTML = `
            <div class="lycoris-container ${this.isMobile ? 'mobile' : 'desktop'}">
                <div class="panel output-panel" id="output-panel">
                    <h3>Output</h3>
                    <div class="content" id="output"></div>
                </div>
                
                <div class="panel stack-panel" id="stack-panel">
                    <h3>Stack</h3>
                    <div class="content" id="stack"></div>
                </div>
                
                <div class="panel input-panel" id="input-panel">
                    <h3>Input</h3>
                    <textarea id="input" placeholder="Enter Lycoris code...
Examples:
5 3 ADD PRINT
[1 2 3] 10 ADD
1 3 DIVIDE
1e61 1e61 MULTIPLY  # 超巨大数演算"></textarea>
                    <div class="button-group">
                        <button id="execute">Execute (Ctrl+Enter)</button>
                        <button id="clear-input">Clear Input</button>
                    </div>
                    <div id="processing-indicator" class="processing-indicator" style="display: none;">
                        Processing large numbers...
                    </div>
                </div>
                
                <div class="panel dictionary-panel" id="dictionary-panel">
                    <h3>Dictionary</h3>
                    <div class="dictionary-content">
                        <table>
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Content</th>
                                </tr>
                            </thead>
                            <tbody id="dictionary-body"></tbody>
                        </table>
                    </div>
                </div>
            </div>
        `;

        this.updateLayout();
    }

    private updateLayout() {
        const container = document.querySelector('.lycoris-container');
        if (!container) return;

        if (this.isMobile) {
            container.classList.add('mobile');
            container.classList.remove('desktop');
            if (this.currentView === 'editor') {
                this.showEditorView();
            } else {
                this.showOutputView();
            }
        } else {
            container.classList.remove('mobile');
            container.classList.add('desktop');
            document.querySelectorAll('.panel').forEach(panel => {
                (panel as HTMLElement).style.display = '';
            });
        }
    }

    private showEditorView() {
        if (!this.isMobile) return;
        
        document.getElementById('output-panel')!.style.display = 'none';
        document.getElementById('stack-panel')!.style.display = 'none';
        document.getElementById('input-panel')!.style.display = 'block';
        document.getElementById('dictionary-panel')!.style.display = 'block';
        this.currentView = 'editor';
    }

    private showOutputView() {
        if (!this.isMobile) return;
        
        document.getElementById('output-panel')!.style.display = 'block';
        document.getElementById('stack-panel')!.style.display = 'block';
        document.getElementById('input-panel')!.style.display = 'none';
        document.getElementById('dictionary-panel')!.style.display = 'none';
        this.currentView = 'output';
    }

    private attachEventListeners() {
        // Execute button
        document.getElementById('execute')?.addEventListener('click', () => {
            this.executeCode();
        });

        // Clear button
        document.getElementById('clear-input')?.addEventListener('click', () => {
            const input = document.getElementById('input') as HTMLTextAreaElement;
            if (input) input.value = '';
        });

        // Ctrl+Enter to execute
        document.getElementById('input')?.addEventListener('keydown', (e) => {
            if (e.ctrlKey && e.key === 'Enter') {
                e.preventDefault();
                this.executeCode();
            }
        });

        // Mobile touch navigation
        if (this.isMobile) {
            ['output-panel', 'stack-panel'].forEach(id => {
                document.getElementById(id)?.addEventListener('click', () => {
                    this.showEditorView();
                });
            });
        }

        // Dictionary word click
        document.addEventListener('click', (e) => {
            const target = e.target as HTMLElement;
            if (target.classList.contains('word-name')) {
                this.insertWordToInput(target.textContent || '');
            }
        });
    }

    private async executeCode() {
        if (this.isProcessing) {
            console.log('Already processing...');
            return;
        }

        const input = document.getElementById('input') as HTMLTextAreaElement;
        if (!input) return;

        const code = input.value.trim();
        if (!code) return;

        try {
            this.isProcessing = true;
            this.showProcessingIndicator(true);
            
            // Web Workerで実行（メインスレッドをブロックしない）
            const result = await this.workerManager.execute(code);
            
            // 結果を保存
            this.currentState = result;
            
            // 表示を更新
            this.updateDisplayFromState(result);
            
            if (this.isMobile) {
                this.showOutputView();
            }
            
            // 永続化
            await this.saveState();
            
        } catch (error: any) {
            this.showError(error.toString());
        } finally {
            this.isProcessing = false;
            this.showProcessingIndicator(false);
        }
    }

    private showProcessingIndicator(show: boolean) {
        const indicator = document.getElementById('processing-indicator');
        if (indicator) {
            indicator.style.display = show ? 'block' : 'none';
        }
        
        const executeBtn = document.getElementById('execute') as HTMLButtonElement;
        if (executeBtn) {
            executeBtn.disabled = show;
            executeBtn.textContent = show ? 'Processing...' : 'Execute (Ctrl+Enter)';
        }
    }

    private updateDisplay() {
        if (this.currentState) {
            this.updateDisplayFromState(this.currentState);
        }
    }

    private updateDisplayFromState(state: any) {
        // Update stack
        const stackElement = document.getElementById('stack');
        if (stackElement) {
            const stack = state.stack || [];
            if (stack.length === 0) {
                stackElement.innerHTML = '<div class="empty">Stack is empty</div>';
            } else {
                const items = stack.map((item: string, index: number) => 
                    `<div class="stack-item">
                        <span class="index">${index}:</span>
                        <span class="value">${this.formatValue(item)}</span>
                    </div>`
                ).join('');
                stackElement.innerHTML = items;
            }
        }

        // Update output
        const outputElement = document.getElementById('output');
        if (outputElement) {
            const output = state.output || '';
            if (!output) {
                outputElement.innerHTML = '<div class="empty">No output</div>';
            } else {
                outputElement.innerHTML = `<pre>${this.escapeHtml(output)}</pre>`;
            }
        }

        // Update dictionary
        const dictBody = document.getElementById('dictionary-body');
        if (dictBody) {
            const dictionary = state.dictionary || [];
            const rows = dictionary.map((word: string[]) => 
                `<tr>
                    <td><span class="word-name" style="color: ${word[2]};">${word[0]}</span></td>
                    <td>${this.escapeHtml(word[1])}</td>
                </tr>`
            ).join('');
            dictBody.innerHTML = rows;
        }
    }

    private insertWordToInput(word: string) {
        const input = document.getElementById('input') as HTMLTextAreaElement;
        if (!input) return;

        const start = input.selectionStart;
        const end = input.selectionEnd;
        const text = input.value;
        
        const before = text.substring(0, start);
        const after = text.substring(end);
        
        input.value = before + word + ' ' + after;
        input.focus();
        
        const newPos = start + word.length + 1;
        input.setSelectionRange(newPos, newPos);
    }

    private showError(error: string) {
        const outputElement = document.getElementById('output');
        if (!outputElement) return;

        outputElement.innerHTML = `<div class="error">Error: ${this.escapeHtml(error)}</div>`;
        
        if (this.isMobile) {
            this.showOutputView();
        }
    }

    private async saveState() {
        if (!this.currentState) return;

        try {
            await this.db.save('dictionary', this.currentState.state);
        } catch (error) {
            console.error('Failed to save state:', error);
        }
    }

    private async loadSavedState() {
        try {
            const state = await this.db.load('dictionary');
            if (state) {
                await this.workerManager.loadState(state);
                const currentState = await this.workerManager.getState();
                this.currentState = currentState;
                // Note: Display will be updated after DOM is ready
            }
        } catch (error) {
            console.error('Failed to load state:', error);
        }
    }

    private formatValue(value: string): string {
        if (value.startsWith('[') && value.endsWith(']')) {
            return `<span class="vector">${value}</span>`;
        } else if (value.startsWith('"') && value.endsWith('"')) {
            return `<span class="string">${value}</span>`;
        } else if (value === 'true' || value === 'false') {
            return `<span class="boolean">${value}</span>`;
        } else if (value === 'nil') {
            return `<span class="nil">${value}</span>`;
        } else {
            // 超巨大数の場合は省略表示
            if (value.includes('/') && value.length > 50) {
                const parts = value.split('/');
                const num = parts[0].substring(0, 20) + '...' + parts[0].substring(parts[0].length - 10);
                const den = parts[1].substring(0, 20) + '...' + parts[1].substring(parts[1].length - 10);
                return `<span class="number" title="${value}">${num}/${den}</span>`;
            }
            return `<span class="number">${value}</span>`;
        }
    }

    private escapeHtml(text: string): string {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
    const app = new LycorisUI();
    await app.init();
});
