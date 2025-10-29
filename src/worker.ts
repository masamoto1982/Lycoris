// Lycoris Web Worker
// 重い計算処理を別スレッドで実行

import init, { Interpreter } from '../pkg/lycoris.js';

let interpreter: Interpreter | null = null;

// メッセージ型定義
interface WorkerMessage {
    type: 'init' | 'execute' | 'getState' | 'loadState';
    data?: any;
    id: number;
}

interface WorkerResponse {
    type: 'success' | 'error';
    data: any;
    id: number;
}

// Worker初期化
async function initializeWorker() {
    try {
        await init();
        interpreter = new Interpreter();
        return { success: true };
    } catch (error) {
        return { success: false, error: String(error) };
    }
}

// コード実行
async function executeCode(code: string) {
    if (!interpreter) {
        throw new Error('Interpreter not initialized');
    }

    try {
        // 実行（重い計算もブロックしない）
        const output = await interpreter.execute(code);
        
        // 結果を収集
        const stack = interpreter.get_stack_json();
        const dictionary = interpreter.get_dictionary_json();
        const state = interpreter.save_state();
        
        return {
            output,
            stack: JSON.parse(stack),
            dictionary: JSON.parse(dictionary),
            state
        };
    } catch (error) {
        throw error;
    }
}

// 状態の取得
function getState() {
    if (!interpreter) {
        throw new Error('Interpreter not initialized');
    }

    return {
        stack: JSON.parse(interpreter.get_stack_json()),
        dictionary: JSON.parse(interpreter.get_dictionary_json()),
        output: interpreter.get_output(),
        state: interpreter.save_state()
    };
}

// 状態の復元
function loadState(state: string) {
    if (!interpreter) {
        throw new Error('Interpreter not initialized');
    }

    try {
        interpreter.load_state(state);
        return { success: true };
    } catch (error) {
        return { success: false, error: String(error) };
    }
}

// メッセージハンドラ
self.addEventListener('message', async (event: MessageEvent<WorkerMessage>) => {
    const { type, data, id } = event.data;
    
    try {
        let result: any;
        
        switch (type) {
            case 'init':
                result = await initializeWorker();
                break;
                
            case 'execute':
                result = await executeCode(data);
                break;
                
            case 'getState':
                result = getState();
                break;
                
            case 'loadState':
                result = loadState(data);
                break;
                
            default:
                throw new Error(`Unknown message type: ${type}`);
        }
        
        const response: WorkerResponse = {
            type: 'success',
            data: result,
            id
        };
        
        self.postMessage(response);
        
    } catch (error) {
        const response: WorkerResponse = {
            type: 'error',
            data: String(error),
            id
        };
        
        self.postMessage(response);
    }
});

// Worker準備完了を通知
self.postMessage({ type: 'ready' });
