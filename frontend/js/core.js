// core.js - ЯДРО СИСТЕМЫ (больше не меняем)

// Состояние
let currentMode = 'common';
let currentCommonFeature = 'info';
let currentVendor = 'potok';
let currentVendorFeature = 'generate';

// ==================== ОБЩИЕ ФИЧИ ====================
window.showCommonFeature = function(feature) {
    currentMode = 'common';
    currentCommonFeature = feature;
    
    window.location.hash = `common/${feature}`;
    highlightActiveMenu(feature);
    renderCommonFeature(feature);
};

// ==================== ФИЧИ КОНТРОЛЛЕРОВ ====================
window.showVendorFeature = function(vendor, feature) {
    currentMode = 'vendor';
    currentVendor = vendor;
    currentVendorFeature = feature;
    
    window.location.hash = `${vendor}/${feature}`;
    highlightActiveMenu(vendor, feature);
    renderVendorFeature(vendor, feature);
};

// ==================== ПОДСВЕТКА МЕНЮ ====================
function highlightActiveMenu(vendorOrFeature, feature) {
    // Сначала убираем подсветку у всех
    document.querySelectorAll('.menu-item, .vendor-header, .feature-item').forEach(el => {
        el.classList.remove('active');
    });
    
    if (feature) {
        // Режим контроллера
        document.querySelector(`[data-vendor="${vendorOrFeature}"][data-feature="${feature}"]`)?.classList.add('active');
        document.querySelector(`[onclick*="toggleVendor('${vendorOrFeature}')"]`)?.classList.add('active');
    } else {
        // Режим общей фичи
        document.querySelector(`[data-common="${vendorOrFeature}"]`)?.classList.add('active');
    }
}

// ==================== РЕНДЕР ====================
async function renderCommonFeature(feature) {
    const content = document.getElementById('feature-content');
    content.innerHTML = `<div class="status info">Загрузка...</div>`;
    
    // Пробуем оба варианта для обратной совместимости
    let renderFunc = window[`renderCommon${capitalize(feature)}`]; // старый стиль (renderCommonGen-scn)
    
    // Если не нашли, пробуем новый стиль: gen-scn → GenScn
    if (!renderFunc && feature.includes('-')) {
        const pascalName = feature.split('-')
            .map(part => part.charAt(0).toUpperCase() + part.slice(1))
            .join('');
        renderFunc = window[`renderCommon${pascalName}`]; // renderCommonGenScn
    }
    
    if (renderFunc) {
        await renderFunc();
    } else {
        content.innerHTML = `<div class="status error">Фича ${feature} в разработке</div>`;
    }
}


// async function renderCommonFeature(feature) {
//     const content = document.getElementById('feature-content');
//     content.innerHTML = `<div class="status info">Загрузка...</div>`;
    
//     const renderFunc = window[`renderCommon${capitalize(feature)}`];
//     if (renderFunc) {
//         await renderFunc();
//     } else {
//         content.innerHTML = `<div class="status error">Фича ${feature} в разработке</div>`;
//     }
// }

async function renderVendorFeature(vendor, feature) {
    const content = document.getElementById('feature-content');
    content.innerHTML = `<div class="status info">Загрузка...</div>`;
    
    const renderFunc = window[`render${capitalize(vendor)}${capitalize(feature)}`];
    if (renderFunc) {
        await renderFunc();
    } else {
        content.innerHTML = `<div class="status error">Фича ${feature} для ${vendor} в разработке</div>`;
    }
}

// ==================== СВОРАЧИВАНИЕ ====================
window.toggleVendor = function(vendor) {
    const features = document.getElementById(`vendor-${vendor}`);
    const toggle = document.querySelector(`[onclick="toggleVendor('${vendor}')"] .vendor-toggle`);
    
    if (features.style.display === 'none') {
        features.style.display = 'block';
        toggle.textContent = '▼';
    } else {
        features.style.display = 'none';
        toggle.textContent = '▶';
    }
};

// ==================== НАВИГАЦИЯ ПО URL ====================
document.addEventListener('DOMContentLoaded', () => {
    const hash = window.location.hash.slice(1);
    
    if (hash) {
        const parts = hash.split('/');
        if (parts[0] === 'common' && parts[1]) {
            showCommonFeature(parts[1]);
        } else if (parts[0] && parts[1]) {
            showVendorFeature(parts[0], parts[1]);
        } else {
            showCommonFeature('info');
        }
    } else {
        showCommonFeature('info');
    }
});

window.addEventListener('hashchange', () => {
    const hash = window.location.hash.slice(1);
    const parts = hash.split('/');
    
    if (parts[0] === 'common' && parts[1]) {
        showCommonFeature(parts[1]);
    } else if (parts[0] && parts[1]) {
        showVendorFeature(parts[0], parts[1]);
    }
});

// ==================== УТИЛИТЫ ====================
function capitalize(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

window.getCurrentVendor = () => currentVendor;
window.getCurrentFeature = () => currentVendorFeature;

// ==================== API ====================
// const API_BASE = window.APP_CONFIG 
//     ? `http://${window.APP_CONFIG.host}:${window.APP_CONFIG.port}`
//     : '';

const API_BASE = window.APP_CONFIG ? window.APP_CONFIG.api_url : '';
       

window.api = {
    get: async (endpoint) => {
        
        const res = await fetch(API_BASE + endpoint);
        return res.json();
    },
    post: async (endpoint, data) => {
        const res = await fetch(API_BASE + endpoint, {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(data)
        });
        return res.json();
    }
};

// ==================== СТАТУСЫ ====================
window.showStatus = function(message, type = 'info') {
    const statusDiv = document.getElementById('status-message');
    if (statusDiv) {
        statusDiv.textContent = message;
        statusDiv.className = `status ${type}`;
        setTimeout(() => {
            statusDiv.textContent = '';
            statusDiv.className = 'status';
        }, 3000);
    }
};