// features/potok/generator.js

window.renderPotokGenerator = function() {
    const content = document.getElementById('feature-content');
    
    content.innerHTML = `
        <h2 style="margin-bottom: 20px;">⚙️ Генератор условий для Поток</h2>
        
        <!-- Документация (сворачиваемая) -->
        <details style="background: #f8f9fa; padding: 15px; border-radius: 8px; border-left: 4px solid #17a2b8; margin-bottom: 20px;" open>
            <summary style="cursor: pointer; font-weight: bold; color: #17a2b8;">
                📚 Документация
            </summary>
            <div style="margin-top: 15px; display: grid; grid-template-columns: 1fr 2fr; gap: 10px; font-size: 14px;">
                <div style="font-family: monospace; background: #e9ecef; padding: 8px; border-radius: 4px;">1-3</div>
                <div style="padding: 8px;">ddr(D1) or ddr(D2) or ddr(D3)</div>
                
                <div style="font-family: monospace; background: #e9ecef; padding: 8px; border-radius: 4px;">or 1-3</div>
                <div style="padding: 8px;">ddr(D1) or ddr(D2) or ddr(D3)</div>
                
                <div style="font-family: monospace; background: #e9ecef; padding: 8px; border-radius: 4px;">&1-3</div>
                <div style="padding: 8px;">ddr(D1) and ddr(D2) and ddr(D3)</div>
                
                <div style="font-family: monospace; background: #e9ecef; padding: 8px; border-radius: 4px;">(or 1-3) and (or 4-6)</div>
                <div style="padding: 8px;">(ddr(D1) or ddr(D2) or ddr(D3)) and (ddr(D4) or ddr(D5) or ddr(D6))</div>
            </div>
            <p style="margin-top: 10px; margin-bottom: 0; color: #666; font-size: 12px;">
                💡 Пробелы не важны: or1-3 = or 1-3 = |1-3
            </p>
        </details>
        
        <!-- Форма генератора -->
        <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
            <div style="margin-bottom: 15px;">
                <label style="font-weight: bold; display: block; margin-bottom: 8px;">
                    📝 Введите выражение:
                </label>
                <textarea 
                    id="inputExpression" 
                    rows="3" 
                    style="width: 100%; padding: 12px; font-family: monospace; font-size: 14px; border: 2px solid #e0e0e0; border-radius: 6px;"
                    placeholder="Например: 1-3 или (or 1-3) and (or 4-6)"
                >1-3</textarea>
            </div>
            
            <button class="btn btn-primary" id="generateBtn" style="width: 100%; padding: 12px; font-size: 16px; margin-bottom: 10px;">
                🚀 Сгенерировать
            </button>
            
            <div style="margin-top: 20px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                    <label style="font-weight: bold;">📊 Результат:</label>
                    <button id="copyBtn" class="btn btn-secondary" style="padding: 5px 15px;">
                        📋 Копировать
                    </button>
                </div>
                <pre id="resultOutput" style="background: #2d2d2d; color: #f8f8f2; padding: 15px; border-radius: 6px; min-height: 60px; white-space: pre-wrap; margin: 0;">ddr(D1) or ddr(D2) or ddr(D3)</pre>
            </div>
        </div>
        
        <!-- Поле для пользователя (опционально) -->
        <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-top: 20px;">
            <label style="font-weight: bold; display: block; margin-bottom: 8px;">
                📝 Ваше поле:
            </label>
            <textarea 
                id="userField" 
                rows="4" 
                style="width: 100%; padding: 12px; font-family: monospace; font-size: 14px; border: 2px solid #e0e0e0; border-radius: 6px;"
                placeholder="Сюда можно вставить результат..."
            ></textarea>
        </div>
    `;

    // Вешаем обработчики
    document.getElementById('generateBtn').addEventListener('click', handleGenerate);
    document.getElementById('copyBtn').addEventListener('click', copyResult);
};

// Функция вызова API
async function handleGenerate() {
    const input = document.getElementById('inputExpression').value.trim();
    const resultEl = document.getElementById('resultOutput');
    
    if (!input) {
        window.showStatus('Введите выражение', 'error');
        return;
    }
    
    // Показываем загрузку
    resultEl.textContent = '⏳ Генерация...';
    window.showStatus('Генерация...', 'info');
    
    try {
        // Вызываем API (используем существующий api объект из core.js)
        const response = await window.api.post('potok/generate-condition', {
            input: input
        });
        
        // Предполагаем, что API возвращает { result: "..." } или { data: "..." }
        const result = response.output || JSON.stringify(response);
        
        resultEl.textContent = result;
        window.showStatus('Готово!', 'success');
        
    } catch (error) {
        console.error('API error:', error);
        
        // Если API недоступно - показываем демо-результат
        resultEl.textContent = generateDemoResult(input);
        window.showStatus('Используется демо-режим (API недоступно)', 'info');
    }
}

// Функция копирования с запасным вариантом
async function copyResult() {
    const resultText = document.getElementById('resultOutput').textContent;
    
    // Пробуем современный Clipboard API
    if (navigator.clipboard && window.isSecureContext) {
        try {
            await navigator.clipboard.writeText(resultText);
            window.showStatus('Скопировано!', 'success');
            return;
        } catch (err) {
            console.warn('Clipboard API failed:', err);
        }
    }
    
    // Запасной вариант: показываем сообщение и выделяем текст
    window.showStatus('Выделите текст и нажмите Ctrl+C (Cmd+C)', 'info');
    
    // Выделяем текст для удобства
    const resultEl = document.getElementById('resultOutput');
    const range = document.createRange();
    range.selectNodeContents(resultEl);
    const selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
}

// Демо-режим (если API недоступно)
function generateDemoResult(input) {
    if (input === '1-3' || input.includes('or 1-3') || input.includes('|1-3')) {
        return 'ddr(D1) or ddr(D2) or ddr(D3)';
    }
    if (input.includes('&1-3')) {
        return 'ddr(D1) and ddr(D2) and ddr(D3)';
    }
    if (input.includes('(')) {
        return '(ddr(D1) or ddr(D2) or ddr(D3)) and (ddr(D4) or ddr(D5) or ddr(D6))';
    }
    return 'ddr(D1) or ddr(D2) or ddr(D3)';
}