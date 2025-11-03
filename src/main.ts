import init, { Interpreter } from '../www/pkg/lycoris.js';

class LycorisUI {
    private interpreter: Interpreter | null = null;
    private historyIndex: number = -1;
    private history: string[] = [];

    async init() {
        try {
            await init();
            this.interpreter = new Interpreter();
            this.setupUI();
            this.showWelcomeMessage();
        } catch (error) {
            console.error('Initialization error:', error);
            this.showError('Failed to initialize Lycoris: ' + error);
        }
    }

    private setupUI() {
        const executeBtn = document.getElementById('execute-btn');
        const clearBtn = document.getElementById('clear-btn');
        const clearStackBtn = document.getElementById('clear-stack-btn');
        const input = document.getElementById('input') as HTMLTextAreaElement;

        executeBtn?.addEventListener('click', () => this.execute());
        clearBtn?.addEventListener('click', () => this.clearOutput());
        clearStackBtn?.addEventListener('click', () => this.clearStack());

        input?.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && e.ctrlKey) {
                e.preventDefault();
                this.execute();
            } else if (e.key === 'ArrowUp' && e.ctrlKey) {
                e.preventDefault();
                this.navigateHistory(-1);
            } else if (e.key === 'ArrowDown' && e.ctrlKey) {
                e.preventDefault();
                this.navigateHistory(1);
            }
        });
    }

    private showWelcomeMessage() {
        const output = document.getElementById('output');
        if (output) {
            output.innerHTML = `
                <div class="welcome">
                    <h2>Welcome to Lycoris</h2>
                    <p>A Stack Rewriting Language</p>
                    <p class="subtitle">Where the Stack IS the Program</p>
                    <div class="examples">
                        <h3>Quick Examples:</h3>
                        <code>5 3 add</code> → 8<br>
                        <code>1 3 div</code> → 1/3 (exact fraction)<br>
                        <code>[1 2 3] 2 @mul</code> → [2 4 6] (map)<br>
                        <code>[1 2 3 4 5] *add</code> → 15 (reduce)<br>
                        <code>5 dup mul</code> → 25 (duplicate and multiply)<br>
                        <code>[dup mul] 'square def</code> → define 'square'<br>
                        <code>7 [square] run</code> → 49<br>
                    </div>
                    <p class="hint">Press Ctrl+Enter to execute</p>
                </div>
            `;
        }
    }

    private execute() {
        if (!this.interpreter) return;

        const input = document.getElementById('input') as HTMLTextAreaElement;
        const code = input.value.trim();

        if (!code) return;

        // 履歴に追加
        this.history.push(code);
        this.historyIndex = this.history.length;

        try {
            const output = this.interpreter.execute(code);
            this.updateDisplay();
            
            // エコー
            this.appendOutput(`> ${code}`);
            
            // 実行結果
            if (output) {
                this.appendOutput(output);
            }

            // 入力をクリア
            input.value = '';
            
        } catch (error: any) {
            this.showError(error.toString());
        }
    }

    private updateDisplay() {
        if (!this.interpreter) return;

        // スタック表示
        const stackElement = document.getElementById('stack');
        if (stackElement) {
            const stackJson = this.interpreter.get_stack_json();
            const stack = JSON.parse(stackJson);
            
            if (stack.length === 0) {
                stackElement.innerHTML = '<div class="empty">Stack is empty</div>';
            } else {
                const items = stack.map((item: string, index: number) => 
                    `<div class="stack-item">
                        <span class="index">${index}:</span>
                        <span class="value">${this.escapeHtml(item)}</span>
                    </div>`
                ).join('');
                stackElement.innerHTML = items;
            }
        }
    }

    private appendOutput(text: string) {
        const output = document.getElementById('output');
        if (!output) return;

        const line = document.createElement('div');
        line.className = 'output-line';
        line.textContent = text;
        output.appendChild(line);
        
        // スクロール
        output.scrollTop = output.scrollHeight;
    }

    private clearOutput() {
        const output = document.getElementById('output');
        if (output) {
            output.innerHTML = '';
        }
        if (this.interpreter) {
            this.interpreter.clear_output();
        }
    }

    private clearStack() {
        if (this.interpreter) {
            // スタックをクリアするために新しいインタープリタを作成
            this.interpreter = new Interpreter();
            this.updateDisplay();
            this.appendOutput('Stack cleared');
        }
    }

    private navigateHistory(direction: number) {
        if (this.history.length === 0) return;

        this.historyIndex += direction;
        
        if (this.historyIndex < 0) {
            this.historyIndex = 0;
        } else if (this.historyIndex >= this.history.length) {
            this.historyIndex = this.history.length;
        }

        const input = document.getElementById('input') as HTMLTextAreaElement;
        if (input) {
            if (this.historyIndex < this.history.length) {
                input.value = this.history[this.historyIndex];
            } else {
                input.value = '';
            }
        }
    }

    private showError(message: string) {
        this.appendOutput(`ERROR: ${message}`);
    }

    private escapeHtml(text: string): string {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
}

// アプリケーション起動
document.addEventListener('DOMContentLoaded', async () => {
    const app = new LycorisUI();
    await app.init();
});
