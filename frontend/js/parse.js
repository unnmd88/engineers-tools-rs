window.renderParse = function() {
    const container = document.getElementById('parse-page');
    
    container.innerHTML = `
        <h2>🔍 Разбор условия</h2>
        <div>
            <input type="text" id="parse-input" value="(A&B)|C" size="30">
            <button onclick="doParse()">Разобрать</button>
        </div>
        <pre id="parse-result">Введите выражение и нажмите кнопку</pre>
    `;
};

window.doParse = async function() {
    const input = document.getElementById('parse-input').value;
    const resultEl = document.getElementById('parse-result');
    
    resultEl.textContent = '⏳ Разбор...';
    
    try {
        const data = await apiRequest('/parse', {
            method: 'POST',
            body: JSON.stringify({ input })
        });
        resultEl.textContent = JSON.stringify(data, null, 2);
    } catch (error) {
        resultEl.textContent = '❌ Ошибка: ' + error.message;
    }
};