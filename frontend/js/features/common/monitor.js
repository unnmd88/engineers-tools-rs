// monitor.js

let activeMonitors = [];
let monitorCounter = 0;

function createMonitor(initialIp = "192.168.1.1", initialInterval = 1) {
    const monitorId = ++monitorCounter;
    let ws = null;
    
    const card = document.createElement('div');
    card.className = 'monitor-card';
    card.style.border = '1px solid #ccc';
    card.style.padding = '10px';
    card.style.marginBottom = '10px';
    card.style.borderRadius = '5px';
    
    card.innerHTML = `
        <div style="display: flex; justify-content: space-between;">
            <h3>Монитор ${monitorId}</h3>
            <button class="delete-btn" style="background: #ff4444; color: white; border: none; border-radius: 4px; cursor: pointer;">Удалить</button>
        </div>
        <div><label>IP:</label> <input class="ip" value="${initialIp}" /></div>
        <div><label>Interval (сек):</label> <input class="interval" value="${initialInterval}" /></div>
        <div><button class="start">Start</button> <button class="stop" disabled>Stop</button></div>
        <pre class="output">Нет данных</pre>
        <hr/>
    `;
    
    const ipInput = card.querySelector('.ip');
    const intervalInput = card.querySelector('.interval');
    const startBtn = card.querySelector('.start');
    const stopBtn = card.querySelector('.stop');
    const output = card.querySelector('.output');
    const deleteBtn = card.querySelector('.delete-btn');
    
    startBtn.onclick = () => {
        console.log(`Start ${monitorId}`);
        
        if (ws) {
            ws.close();
            ws = null;
        }
        
        ws = new WebSocket(`ws://${window.APP_CONFIG.host}:${window.APP_CONFIG.port}/api/v1/common/ws/monitor`);
        
        ws.onopen = () => {
            ws.send(JSON.stringify({
                ip: ipInput.value,
                interval: Number(intervalInput.value)
            }));
            ipInput.disabled = true;
            intervalInput.disabled = true;
            startBtn.disabled = true;
            stopBtn.disabled = false;
            output.textContent = 'Подключено...';
        };
        
        ws.onmessage = (e) => {
            output.textContent = e.data;
        };
        
        ws.onerror = (err) => {
            output.textContent = `Ошибка: ${err}`;
        };
        
        ws.onclose = () => {
            console.log(`Close ${monitorId}`);
            ipInput.disabled = false;
            intervalInput.disabled = false;
            startBtn.disabled = false;
            stopBtn.disabled = true;
            ws = null;
            output.textContent = 'Соединение закрыто';
        };
    };
    
    stopBtn.onclick = () => {
        console.log(`Stop ${monitorId}`);
        if (ws) ws.close();
    };
    
    deleteBtn.onclick = () => {
        console.log(`Delete ${monitorId}`);
        if (ws) ws.close();
        card.remove();
        activeMonitors = activeMonitors.filter(m => m.id !== monitorId);
    };
    
    return { id: monitorId, card, destroy: () => { if (ws) ws.close(); card.remove(); } };
}

window.renderCommonMonitor = async function () {
    console.log("renderCommonMonitor");
    
    const content = document.getElementById('feature-content');
    content.innerHTML = `
        <h2>📡 Мониторы ДК</h2>
        <button id="add-monitor">+ Добавить монитор</button>
        <div id="monitors-container"></div>
    `;
    
    const container = document.getElementById('monitors-container');
    
    for (const m of activeMonitors) m.destroy();
    activeMonitors = [];
    monitorCounter = 0;
    
    const first = createMonitor();
    container.appendChild(first.card);
    activeMonitors.push(first);
    
    document.getElementById('add-monitor').onclick = () => {
        const newMon = createMonitor();
        container.appendChild(newMon.card);
        activeMonitors.push(newMon);
    };
    
    return {
        cleanup: () => {
            console.log("Cleanup all monitors");
            for (const m of activeMonitors) m.destroy();
            activeMonitors = [];
            monitorCounter = 0;
        }
    };
};