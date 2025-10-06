import type { LycorisInterpreter } from './pkg/lycoris_core';

declare global {
    interface Window {
        lycorisInterpreter: LycorisInterpreter;
    }
}

async function init() {
    try {
        const wasm = await import('./pkg/lycoris_core.js');
        await wasm.default();
        window.lycorisInterpreter = new wasm.LycorisInterpreter();
        
        setupEventListeners();
        updateDisplay();
        
        console.log('Lycoris initialized');
    } catch (error) {
        console.error('Failed to initialize:', error);
    }
}

function setupEventListeners() {
    document.getElementById('run-btn')?.addEventListener('click', runCode);
    document.getElementById('clear-btn')?.addEventListener('click', clearInput);
    
    document.getElementById('code-input')?.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' && e.shiftKey) {
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
                outputDisplay.textContent = result.output || 'OK';
                input.value = '';
            } else {
                outputDisplay.textContent = `Error: ${result.message}`;
            }
        }
        
        updateDisplay();
    } catch (error) {
        console.error('Execution error:', error);
    }
}

function clearInput() {
    const input = document.getElementById('code-input') as HTMLTextAreaElement;
    input.value = '';
    input.focus();
}

function updateDisplay() {
    // Update stack
    const stack = window.lycorisInterpreter.get_stack();
    const stackDisplay = document.getElementById('stack-display');
    if (stackDisplay) {
        if (stack.length === 0) {
            stackDisplay.textContent = '(empty)';
        } else {
            stackDisplay.innerHTML = stack.map((item: any) => 
                `<div class="stack-item">${formatValue(item)}</div>`
            ).join('');
        }
    }
    
    // Update dictionary
    const words = window.lycorisInterpreter.get_custom_words_info();
    const dictDisplay = document.getElementById('custom-words-display');
    if (dictDisplay) {
        dictDisplay.innerHTML = words.map((word: any) => 
            `<button class="word-button" onclick="insertWord('${word[0]}')">${word[0]}</button>`
        ).join('');
    }
}

function formatValue(value: any): string {
    if (value.type === 'vector') {
        const items = value.value.map((v: any) => formatValue(v)).join(' ');
        return `[${items}]`;
    }
    if (value.type === 'number') {
        const { numerator, denominator } = value.value;
        return denominator === '1' ? numerator : `${numerator}/${denominator}`;
    }
    if (value.type === 'string') return `'${value.value}'`;
    if (value.type === 'boolean') return value.value ? 'TRUE' : 'FALSE';
    if (value.type === 'nil') return 'NIL';
    return '?';
}

(window as any).insertWord = (word: string) => {
    const input = document.getElementById('code-input') as HTMLTextAreaElement;
    const pos = input.selectionStart;
    const text = input.value;
    input.value = text.slice(0, pos) + word + ' ' + text.slice(pos);
    input.focus();
};

// Initialize on load
document.addEventListener('DOMContentLoaded', init);
