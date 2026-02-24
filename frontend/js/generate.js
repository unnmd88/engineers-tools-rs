
window.renderGenerate = function() {
    const container = document.getElementById('generate-page');
    
    container.innerHTML = `
        <h2>⚙️ Генерация условия</h2>
        <div>
            <input type="text" id="generate-input" value="(A&B)|C" size="30">
            <button onclick="doGenerate()">Сгенерировать</button>
        </div>
        <pre id="generate-result">Введите выражение и нажмите кнопку</pre>
    `;
};

window.doGenerate = async function() {
    const input = document.getElementById('generate-input').value;
    const resultEl = document.getElementById('generate-result');
    
    resultEl.textContent = '⏳ Генерация...';
    
    try {
        const data = await apiRequest('/generate', {
            method: 'POST',
            body: JSON.stringify({ input })
        });
        resultEl.textContent = JSON.stringify(data, null, 2);
    } catch (error) {
        resultEl.textContent = '❌ Ошибка: ' + error.message;
    }
};