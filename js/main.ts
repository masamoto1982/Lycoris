import init, { LycorisInterpreter } from './pkg/lycoris_core.js';
import type { Value } from './pkg/lycoris_core.d';

declare global {
    interface Window {
        lycorisInterpreter: LycorisInterpreter;
        insertWord: (word: string) => void;
    }
}

async function main() {
    try {
        await init();
        window.lycorisInterpreter = new LycorisInterpreter();
        
        setupEventListeners();
        updateDisplay();
        
        console.log('Lycoris initialized');
    } catch (error) {
        console.error('Failed to initialize Wasm module:', error);
        const outputDisplay = document.getElementById('output-display');
        if (outputDisplay) {
            outputDisplay.textContent = 'Error: Failed to load WebAssembly module. Check the console for details.';
        }
    }
}

function setupEventListeners() {
    document.getElementById('run-btn')?.addEventListener('click', runCode);
    document.getElementById('clear-btn')?.addEventListener('click', clearInput);
    
    document.getElementById('code-input')?.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
            e.preventDefault();
            runCode();
        }
    });
}

async function runCode() {
    const input = document.getElementById('code-input') as HTMLTextAreaElement;
    const code = input.value.trim();
    
    if (!code) return;
    
    try {
        const result = await window.lycorisInterpreter.execute(code);
        
        const outputDisplay = document.getElementById('output-display');
        if (outputDisplay) {
            if (result.status === 'OK') {
                outputDisplay.textContent = result.output || '(No output)';
                // Don't clear input to allow for iterative development
                // input.value = ''; 
            } else {
                outputDisplay.textContent = `Error: ${result.message}`;
            }
        }
        
        updateDisplay();
    } catch (error) {
        console.error('Execution error:', error);
        const outputDisplay = document.getElementById('output-display');
        if (outputDisplay) {
            outputDisplay.textContent = `Execution Error: ${error}`;
        }
    }
}

function clearInput() {
    const input = document.getElementById('code-input') as HTMLTextAreaElement;
    input.value = '';
    const outputDisplay = document.getElementById('output-display');
    if(outputDisplay) outputDisplay.textContent = '';
    window.lycorisInterpreter.reset();
    updateDisplay();
    input.focus();
}

function updateDisplay() {
    // Update stack
    const stack = window.lycorisInterpreter.get_stack() as Value[];
    const stackDisplay = document.getElementById('stack-display');
    if (stackDisplay) {
        stackDisplay.innerHTML = stack.length === 0 
            ? '(empty)'
            : stack.map(item => `<div class="stack-item">${formatValue(item)}</div>`).join('');
    }
    
    // Update dictionary
    const words = window.lycorisInterpreter.get_custom_words_info() as [string, string][];
    const dictDisplay = document.getElementById('custom-words-display');
    if (dictDisplay) {
        dictDisplay.innerHTML = words.map(([name, body]) => 
            `<button class="word-button" title="${body}" onclick="insertWord('${name}')">${name}</button>`
        ).join('');
    }
}

function formatValue(value: Value): string {
    if (!value || !value.type) {
        // Handle potential inconsistencies if the wasm output is not as expected
        return '?';
    }
    switch (value.type) {
        case 'number':
            const { numerator, denominator } = value.value as { numerator: string, denominator: string };
            return denominator === '1' ? numerator : `${numerator}/${denominator}`;
        case 'string':
            return `'${value.value}'`;
        case 'boolean':
            return value.value ? 'TRUE' : 'FALSE';
        case 'vector':
            const items = (value.value as Value[]).map(v => formatValue(v)).join(' ');
            return `[ ${items} ]`;
        case 'symbol':
            return String(value.value);
        case 'nil':
            return 'NIL';
        default:
            return '?';
    }
}


window.insertWord = (word: string) => {
    const input = document.getElementById('code-input') as HTMLTextAreaElement;
    const pos = input.selectionStart;
    const text = input.value;
    input.value = text.slice(0, pos) + word + ' ' + text.slice(pos);
    input.focus();
    input.selectionStart = pos + word.length + 1;
    input.selectionEnd = pos + word.length + 1;
};

// Initialize on load
document.addEventListener('DOMContentLoaded', main);
