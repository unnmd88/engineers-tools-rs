// features/scn/gen-scn.js

window.renderCommonGenScn = function() {
    const content = document.getElementById('feature-content');
    
    content.innerHTML = `
        <h2 style="margin-bottom: 20px;">📄 Генератор SCN</h2>
        
        <!-- Документация (пустая, сворачиваемая) -->
        <details style="background: #f8f9fa; padding: 15px; border-radius: 8px; border-left: 4px solid #28a745; margin-bottom: 20px;">
            <summary style="cursor: pointer; font-weight: bold; color: #28a745;">
                📚 Документация
            </summary>
            <div style="margin-top: 15px; color: #666;">
                <p>Документация появится позже...</p>
            </div>
        </details>
        
        <!-- Форма генератора -->
        <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
            <div style="margin-bottom: 15px;">
                <label style="font-weight: bold; display: block; margin-bottom: 8px;">
                    📝 Введите данные для SCN:
                </label>
                <textarea 
                    id="scnInput" 
                    rows="4" 
                    style="width: 100%; padding: 12px; font-family: monospace; font-size: 14px; border: 2px solid #e0e0e0; border-radius: 6px;"
                    placeholder="Введите параметры SCN..."
                ></textarea>
            </div>
            
            <button class="btn btn-primary" id="generateScnBtn" style="width: 100%; padding: 12px; font-size: 16px; margin-bottom: 10px;">
                🚀 Сгенерировать SCN
            </button>
            
            <div style="margin-top: 20px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                    <label style="font-weight: bold;">📊 Результат:</label>
                    <button id="copyScnBtn" class="btn btn-secondary" style="padding: 5px 15px;">
                        📋 Копировать
                    </button>
                </div>
                <pre id="scnResult" style="background: #2d2d2d; color: #f8f8f2; padding: 15px; border-radius: 6px; min-height: 60px; white-space: pre-wrap; margin: 0;">Здесь появится результат...</pre>
            </div>
        </div>
        
        <!-- Поле для пользователя -->
        <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-top: 20px;">
            <label style="font-weight: bold; display: block; margin-bottom: 8px;">
                📝 Ваше поле:
            </label>
            <textarea 
                id="scnUserField" 
                rows="4" 
                style="width: 100%; padding: 12px; font-family: monospace; font-size: 14px; border: 2px solid #e0e0e0; border-radius: 6px;"
                placeholder="Сюда можно вставить результат..."
            ></textarea>
        </div>
    `;

    // Вешаем обработчики
    document.getElementById('generateScnBtn').addEventListener('click', handleScnGenerate);
    document.getElementById('copyScnBtn').addEventListener('click', copyScnResult);
};

// Функция вызова API
async function handleScnGenerate() {
    const input = document.getElementById('scnInput').value.trim();
    const resultEl = document.getElementById('scnResult');
    
    if (!input) {
        window.showStatus('Введите данные для SCN', 'error');
        return;
    }
    
    resultEl.textContent = '⏳ Генерация...';
    window.showStatus('Генерация SCN...', 'info');
    
    try {
        // ✅ Чисто! Никакой возни с response, ok, json()
        const data = await window.api.post('common/scn-generations', {
            input: input
        });
        
        // ✅ Просто используем данные
        resultEl.textContent = data.output;
        window.showStatus('Готово!', 'success');
        
    } catch (error) {
        // ✅ Все ошибки уже обработаны в api.post
        resultEl.textContent = `❌ ${error.message}`;
        window.showStatus('Ошибка', 'error');
    }
}

// Функция копирования
async function copyScnResult() {
    const resultText = document.getElementById('scnResult').textContent;
    
    if (navigator.clipboard && window.isSecureContext) {
        try {
            await navigator.clipboard.writeText(resultText);
            window.showStatus('Скопировано!', 'success');
            return;
        } catch (err) {
            console.warn('Clipboard API failed:', err);
        }
    }
    
    window.showStatus('Выделите текст и нажмите Ctrl+C (Cmd+C)', 'info');
    
    const resultEl = document.getElementById('scnResult');
    const range = document.createRange();
    range.selectNodeContents(resultEl);
    const selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
}

// Демо-режим (заглушка)
function generateScnDemoResult(input) {
    if (!input) return '{}';
    
    // Простая заглушка, возвращает введенный текст в виде JSON
    try {
        // Если ввели JSON - форматируем его
        if (input.trim().startsWith('{') || input.trim().startsWith('[')) {
            const parsed = JSON.parse(input);
            return JSON.stringify(parsed, null, 2);
        }
    } catch {
        // Если не JSON - возвращаем как есть в объекте
    }
    
    return JSON.stringify({
        input: input,
        generated: true,
        timestamp: new Date().toISOString()
    }, null, 2);
}