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
                
                <div style="font-family: monospace; background: #e9ecef; padding: 8px; border-radius: 4px;">((|1-3)&(|4-6))|(&7-9)</div>
                <div style="padding: 8px;">сложное выражение с символами</div>
            </div>
            <p style="margin-top: 10px; margin-bottom: 0; color: #666; font-size: 12px;">
                💡 Пробелы не важны: or1-3 = or 1-3 = |1-3
            </p>
        </details>
        
        <!-- Форма генератора -->
        <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
            <div class="form-group" style="margin-bottom: 15px;">
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
            
            <button class="btn btn-primary" id="generateBtn" style="width: 100%; padding: 12px; font-size: 16px;">
                🚀 Сгенерировать условие
            </button>
            
            <div style="margin-top: 20px;">
                <label style="font-weight: bold; display: block; margin-bottom: 8px;">
                    📊 Результат:
                </label>
                <pre id="resultOutput" style="background: #2d2d2d; color: #f8f8f2; padding: 15px; border-radius: 6px; min-height: 60px; white-space: pre-wrap;">ddr(D1) or ddr(D2) or ddr(D3)</pre>
            </div>
        </div>
        
        <!-- Поле для пользователя -->
        <div style="background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-top: 20px;">
            <label style="font-weight: bold; display: block; margin-bottom: 8px;">
                📝 Ваше поле:
            </label>
            <textarea 
                id="userField" 
                rows="4" 
                style="width: 100%; padding: 12px; font-family: monospace; font-size: 14px; border: 2px solid #e0e0e0; border-radius: 6px;"
                placeholder="Сюда можно копировать результаты и редактировать..."
            ></textarea>
        </div>
    `;

    document.getElementById('generateBtn').addEventListener('click', handleGenerate);
};

function handleGenerate() {
    const input = document.getElementById('inputExpression').value.trim();
    const resultEl = document.getElementById('resultOutput');
    
    if (!input) {
        showStatus('Введите выражение', 'error');
        return;
    }
    
    showStatus('Генерация...', 'info');
    
    // Здесь будет вызов API
    setTimeout(() => {
        const result = generateCondition(input);
        resultEl.textContent = result;
        showStatus('Готово!', 'success');
    }, 500);
}

function generateCondition(input) {
    // Простая логика для демо
    if (input === '1-3') return 'ddr(D1) or ddr(D2) or ddr(D3)';
    if (input.includes('&')) return 'ddr(D1) and ddr(D2) and ddr(D3)';
    if (input.includes('(')) return '(ddr(D1) or ddr(D2) or ddr(D3)) and (ddr(D4) or ddr(D5) or ddr(D6))';
    return 'ddr(D1) or ddr(D2) or ddr(D3)';
}