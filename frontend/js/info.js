window.renderInfo = function() {
    const container = document.getElementById('info-page');
    
    container.innerHTML = `
        <h2>ℹ️ Информация о системе</h2>
        <button onclick="loadInfo()">Загрузить информацию</button>
        <pre id="info-result">Нажмите кнопку для загрузки</pre>
    `;
};

window.loadInfo = async function() {
    const resultEl = document.getElementById('info-result');
    resultEl.textContent = '⏳ Загрузка...';
    
    try {
        const data = await apiRequest('/info');
        resultEl.textContent = JSON.stringify(data, null, 2);
    } catch (error) {
        resultEl.textContent = '❌ Ошибка: ' + error.message;
    }
};