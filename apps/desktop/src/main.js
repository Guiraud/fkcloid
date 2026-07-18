const { invoke } = window.__TAURI__.core;

// Application State
let activeFolderId = 'root';
let activeFolderName = 'Racine';
let modalSelectedFolderId = 'root';
let modalSelectedFolderName = 'Racine';
let cachedDocumentTree = { entries: [], trash: [] };
let uploadQueue = [];
let isUploading = false;
let pendingFilesToConfirm = [];

// DOM Elements
let loginScreen;
let appScreen;
let loginForm;
let btnLoginSubmit;
let loginError;
let userUsernameDisplay;
let serverHostDisplay;
let btnLogout;
let btnSettings;
let btnSettingsClose;
let settingsModal;
let settingAutostart;
let settingContextmenu;
let folderTree;
let modalFolderTree;
let activeFolderNameDisplay;
let btnSync;
let dropZone;
let queueList;
let btnClearQueue;
let btnNewFolder;
let dragOverlay;

let folderModal;
let btnModalCancel;
let btnModalConfirm;

// Helper to escape HTML characters
function escapeHtml(str) {
  if (!str) return '';
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

// Check auth status on startup
async function init() {
  try {
    const isAuthed = await invoke('check_auth');
    if (isAuthed) {
      const config = await invoke('get_config');
      if (config) {
        showDashboard(config);
        
        // Handle pending upload on startup
        try {
          const pendingFile = await invoke('get_pending_upload');
          if (pendingFile) {
            handleFileDrop([pendingFile]);
          }
        } catch (e) {
          console.error("Failed to check pending upload:", e);
        }
      } else {
        showLogin();
      }
    } else {
      showLogin();
    }
  } catch (err) {
    console.error('Error during init:', err);
    showLogin();
  }
}

function showLogin() {
  loginScreen.classList.remove('hidden');
  appScreen.classList.add('hidden');
}

function showDashboard(config) {
  loginScreen.classList.add('hidden');
  appScreen.classList.remove('hidden');
  
  userUsernameDisplay.textContent = config.username;
  serverHostDisplay.textContent = config.host;
  
  // Set default state
  activeFolderId = 'root';
  activeFolderName = 'Racine';
  activeFolderNameDisplay.textContent = activeFolderName;
  
  refreshTree();
}

// API: Refresh document tree and render folders
async function refreshTree() {
  try {
    const tree = await invoke('get_documents');
    cachedDocumentTree = tree;
    renderFolderTree();
  } catch (err) {
    console.error('Failed to fetch documents:', err);
  }
}

// Render folder hierarchy
function renderFolderTree() {
  const treeHtml = buildFolderTreeHtml(cachedDocumentTree.Entries, activeFolderId, false);
  const rootActive = activeFolderId === 'root' ? 'active' : '';
  
  folderTree.innerHTML = `
    <div class="folder-row ${rootActive}" data-id="root">
      <span class="folder-icon">🏡</span>
      <span class="folder-name">Racine</span>
    </div>
    ${treeHtml}
  `;

  // Attach click events
  folderTree.querySelectorAll('.folder-row').forEach(row => {
    row.addEventListener('click', (e) => {
      e.stopPropagation();
      const id = row.getAttribute('data-id');
      selectFolder(id);
    });
  });
}

// Render folder hierarchy in modal
function renderModalFolderTree() {
  const treeHtml = buildFolderTreeHtml(cachedDocumentTree.Entries, modalSelectedFolderId, true);
  const rootActive = modalSelectedFolderId === 'root' ? 'active' : '';
  
  modalFolderTree.innerHTML = `
    <div class="folder-row ${rootActive}" data-id="root">
      <span class="folder-icon">🏡</span>
      <span class="folder-name">Racine</span>
    </div>
    ${treeHtml}
  `;

  // Attach click events
  modalFolderTree.querySelectorAll('.folder-row').forEach(row => {
    row.addEventListener('click', (e) => {
      e.stopPropagation();
      const id = row.getAttribute('data-id');
      modalSelectedFolderId = id;
      modalSelectedFolderName = findFolderName(cachedDocumentTree.Entries, id) || 'Racine';
      renderModalFolderTree();
    });
  });
}

// Build folder tree recursively
function buildFolderTreeHtml(entries, activeId, isModal) {
  let html = '<ul>';
  let hasFolders = false;
  for (const entry of entries) {
    if (entry.isFolder) {
      hasFolders = true;
      const isActive = entry.id === activeId ? 'active' : '';
      html += `
        <li class="folder-item-li" data-id="${entry.id}">
          <div class="folder-row ${isActive}" data-id="${entry.id}">
            <span class="folder-icon">📁</span>
            <span class="folder-name">${escapeHtml(entry.name)}</span>
          </div>
          ${entry.children && entry.children.some(c => c.isFolder) ? buildFolderTreeHtml(entry.children, activeId, isModal) : ''}
        </li>
      `;
    }
  }
  html += '</ul>';
  return hasFolders ? html : '';
}

// Find folder name by ID recursively
function findFolderName(entries, id) {
  if (id === 'root') return 'Racine';
  for (const entry of entries) {
    if (entry.id === id) return entry.name;
    if (entry.children && entry.children.length > 0) {
      const name = findFolderName(entry.children, id);
      if (name) return name;
    }
  }
  return null;
}

// Set active folder
function selectFolder(id) {
  activeFolderId = id;
  activeFolderName = findFolderName(cachedDocumentTree.Entries, id) || 'Racine';
  activeFolderNameDisplay.textContent = activeFolderName;
  
  renderFolderTree();
}

// Inline creation of folder
function showInlineCreateFolderInput() {
  // Check if input already exists to prevent duplicate UI elements
  if (document.querySelector('.new-folder-input-row')) return;
  
  // Find where to append the input. 
  // If active folder is root, append at the end of the folderTree.
  // Else, append under the active folder's <li>
  let targetLi = null;
  if (activeFolderId !== 'root') {
    targetLi = folderTree.querySelector(`.folder-item-li[data-id="${activeFolderId}"]`);
  }

  const inputRow = document.createElement('div');
  inputRow.className = 'new-folder-input-row';
  inputRow.innerHTML = `
    <input type="text" class="new-folder-input" placeholder="Nom du dossier..." required />
    <button class="new-folder-btn" id="btn-save-folder">✓</button>
    <button class="new-folder-btn new-folder-cancel" id="btn-cancel-folder">✗</button>
  `;

  if (targetLi) {
    // Append under current folder list (create one if not present)
    let ul = targetLi.querySelector('ul');
    if (!ul) {
      ul = document.createElement('ul');
      targetLi.appendChild(ul);
    }
    const li = document.createElement('li');
    li.appendChild(inputRow);
    ul.appendChild(li);
  } else {
    // Append to top-level folderTree container
    const div = document.createElement('div');
    div.appendChild(inputRow);
    folderTree.appendChild(div);
  }

  const input = inputRow.querySelector('.new-folder-input');
  input.focus();

  // Cancel Handler
  inputRow.querySelector('#btn-cancel-folder').addEventListener('click', () => {
    inputRow.remove();
  });

  // Save Handler
  const saveFolder = async () => {
    const name = input.value.trim();
    if (!name) return;
    
    inputRow.innerHTML = `<span class="spinner" style="margin-left: 10px;"></span>`;
    
    try {
      await invoke('create_folder', { parentId: activeFolderId, name });
      await refreshTree();
    } catch (err) {
      alert(`Erreur de création: ${err}`);
      await refreshTree();
    }
  };

  inputRow.querySelector('#btn-save-folder').addEventListener('click', saveFolder);
  input.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      saveFolder();
    } else if (e.key === 'Escape') {
      inputRow.remove();
    }
  });
}

// Drag & Drop Handlers
function handleFileDrop(paths) {
  if (!paths || paths.length === 0) return;
  pendingFilesToConfirm = paths;
  
  // Setup modal default state
  modalSelectedFolderId = activeFolderId;
  modalSelectedFolderName = activeFolderName;
  
  // Show confirmation modal
  folderModal.classList.remove('hidden');
  renderModalFolderTree();
}

function confirmImport() {
  folderModal.classList.add('hidden');
  
  for (const path of pendingFilesToConfirm) {
    // Extract filename
    const filename = path.replace(/^.*[\\\/]/, '');
    const extension = filename.split('.').pop().toLowerCase();
    
    const queueItem = {
      id: Math.random().toString(36).substring(2, 9),
      name: filename,
      path: path,
      folderId: modalSelectedFolderId,
      folderName: modalSelectedFolderName,
      status: 'pending',
      error: ''
    };
    
    if (extension !== 'pdf' && extension !== 'epub' && extension !== 'md' && extension !== 'markdown') {
      queueItem.status = 'failed';
      queueItem.error = 'Seuls les fichiers PDF, EPUB et Markdown sont acceptés.';
    }
    
    uploadQueue.push(queueItem);
  }
  
  pendingFilesToConfirm = [];
  renderQueue();
  processUploadQueue();
}

// Process Upload Queue sequentially
async function processUploadQueue() {
  if (isUploading) return;
  
  const nextItem = uploadQueue.find(item => item.status === 'pending');
  if (!nextItem) {
    isUploading = false;
    return;
  }
  
  isUploading = true;
  nextItem.status = 'uploading';
  renderQueue();
  
  try {
    await invoke('upload_document', { 
      parentId: nextItem.folderId, 
      filePath: nextItem.path 
    });
    nextItem.status = 'success';
  } catch (err) {
    nextItem.status = 'failed';
    nextItem.error = err.toString();
  }
  
  renderQueue();
  isUploading = false;
  
  // Call next item
  setTimeout(processUploadQueue, 50);
}

// Render Upload Queue
function renderQueue() {
  if (uploadQueue.length === 0) {
    queueList.innerHTML = `
      <div class="queue-empty" id="queue-empty">
        <span>Aucun envoi en cours ou terminé</span>
      </div>
    `;
    return;
  }
  
  let html = '';
  for (const item of uploadQueue) {
    let statusIconHtml = '';
    let statusTextClass = '';
    let statusText = '';
    let retryBtnHtml = '';
    
    switch (item.status) {
      case 'pending':
        statusIconHtml = '⏳';
        statusTextClass = 'status-pending';
        statusText = 'En attente';
        break;
      case 'uploading':
        statusIconHtml = '<span class="spinner"></span>';
        statusTextClass = 'status-uploading';
        statusText = 'Envoi en cours';
        break;
      case 'success':
        statusIconHtml = '✅';
        statusTextClass = 'status-success';
        statusText = 'Succès';
        break;
      case 'failed':
        statusIconHtml = '❌';
        statusTextClass = 'status-failed';
        statusText = 'Échec';
        retryBtnHtml = `<button class="btn-secondary" style="padding: 4px 8px; font-size: 0.7rem;" onclick="window.retryUpload('${item.id}')">Réessayer</button>`;
        break;
    }
    
    let fileIcon = '📄';
    const lowerName = item.name.toLowerCase();
    if (lowerName.endsWith('.epub')) {
      fileIcon = '📖';
    } else if (lowerName.endsWith('.md') || lowerName.endsWith('.markdown')) {
      fileIcon = '📝';
    }
    const errorDetails = item.error ? `<div style="color: #f43f5e; font-size: 0.75rem; margin-top: 4px; max-width: 300px; white-space: normal; word-break: break-all;">${escapeHtml(item.error)}</div>` : '';
    
    html += `
      <div class="queue-item" data-id="${item.id}">
        <div class="item-left">
          <span class="item-file-icon">${fileIcon}</span>
          <div class="item-info">
            <div class="item-name" title="${escapeHtml(item.name)}">${escapeHtml(item.name)}</div>
            <div class="item-dest">Dossier: ${escapeHtml(item.folderName)}</div>
            ${errorDetails}
          </div>
        </div>
        <div class="item-right">
          ${retryBtnHtml}
          <div style="display: flex; align-items: center; gap: 6px;">
            <span>${statusIconHtml}</span>
            <span class="item-status-text ${statusTextClass}">${statusText}</span>
          </div>
        </div>
      </div>
    `;
  }
  
  queueList.innerHTML = html;
}

// Global retry helper
window.retryUpload = function(id) {
  const item = uploadQueue.find(i => i.id === id);
  if (item && item.status === 'failed') {
    item.status = 'pending';
    item.error = '';
    renderQueue();
    processUploadQueue();
  }
};

// Event Bindings
window.addEventListener("DOMContentLoaded", () => {
  loginScreen = document.querySelector("#login-screen");
  appScreen = document.querySelector("#app-screen");
  loginForm = document.querySelector("#login-form");
  btnLoginSubmit = document.querySelector("#btn-login-submit");
  loginError = document.querySelector("#login-error");
  userUsernameDisplay = document.querySelector("#user-username-display");
  serverHostDisplay = document.querySelector("#server-host-display");
  btnLogout = document.querySelector("#btn-logout");
  btnSettings = document.querySelector("#btn-settings");
  btnSettingsClose = document.querySelector("#btn-settings-close");
  settingsModal = document.querySelector("#settings-modal");
  settingAutostart = document.querySelector("#setting-autostart");
  settingContextmenu = document.querySelector("#setting-contextmenu");
  
  folderTree = document.querySelector("#folder-tree");
  modalFolderTree = document.querySelector("#modal-folder-tree");
  activeFolderNameDisplay = document.querySelector("#active-folder-name");
  btnSync = document.querySelector("#btn-sync");
  dropZone = document.querySelector("#drop-zone");
  queueList = document.querySelector("#queue-list");
  btnClearQueue = document.querySelector("#btn-clear-queue");
  btnNewFolder = document.querySelector("#btn-new-folder");
  dragOverlay = document.querySelector("#drag-overlay");
  
  folderModal = document.querySelector("#folder-modal");
  btnModalCancel = document.querySelector("#btn-modal-cancel");
  btnModalConfirm = document.querySelector("#btn-modal-confirm");

  // Login Form Submit
  loginForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    loginError.style.display = 'none';
    btnLoginSubmit.disabled = true;
    btnLoginSubmit.innerHTML = `<span class="spinner"></span> <span>Connexion...</span>`;
    
    const host = document.querySelector("#input-host").value.trim();
    const username = document.querySelector("#input-username").value.trim();
    const password = document.querySelector("#input-password").value;
    
    try {
      await invoke('login', { host, username, password });
      showDashboard({ host, username });
    } catch (err) {
      loginError.textContent = `Erreur de connexion : ${err}`;
      loginError.style.display = 'block';
    } finally {
      btnLoginSubmit.disabled = false;
      btnLoginSubmit.textContent = 'Se connecter';
    }
  });

  // Logout Click
  btnLogout.addEventListener("click", async () => {
    try {
      await invoke('logout');
      showLogin();
    } catch (err) {
      console.error('Logout failed:', err);
    }
  });

  // Sync Click
  btnSync.addEventListener("click", async () => {
    btnSync.disabled = true;
    const oldText = btnSync.textContent;
    btnSync.innerHTML = `<span class="spinner"></span> <span>Synchronisation...</span>`;
    
    try {
      await invoke('sync_tablet');
      btnSync.innerHTML = `✅ Synchronisé`;
    } catch (err) {
      alert(`Erreur de synchronisation : ${err}`);
      btnSync.textContent = oldText;
    } finally {
      setTimeout(() => {
        btnSync.disabled = false;
        btnSync.textContent = '🔄 Synchroniser tablette';
      }, 2000);
    }
  });

  // Inline folder creation click
  btnNewFolder.addEventListener('click', showInlineCreateFolderInput);

  // Clear Upload Queue
  btnClearQueue.addEventListener('click', () => {
    // Only keep items that are currently uploading or pending
    uploadQueue = uploadQueue.filter(item => item.status === 'pending' || item.status === 'uploading');
    renderQueue();
  });

  // Modal actions
  btnModalCancel.addEventListener('click', () => {
    folderModal.classList.add('hidden');
    pendingFilesToConfirm = [];
  });
  
  btnModalConfirm.addEventListener('click', confirmImport);
  
  // Settings Button Click
  btnSettings.addEventListener('click', async () => {
    try {
      const config = await invoke('get_config');
      if (config) {
        settingAutostart.checked = !!config.autostart;
        settingContextmenu.checked = !!config.contextmenu;
      }
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
    settingsModal.classList.remove('hidden');
  });

  btnSettingsClose.addEventListener('click', () => {
    settingsModal.classList.add('hidden');
  });

  settingAutostart.addEventListener('change', saveSettings);
  settingContextmenu.addEventListener('change', saveSettings);

  async function saveSettings() {
    const autostart = settingAutostart.checked;
    const contextmenu = settingContextmenu.checked;
    try {
      await invoke('save_settings', { autostart, contextmenu });
    } catch (err) {
      alert(`Erreur de sauvegarde des paramètres : ${err}`);
    }
  }

  // Click on dropzone opens native OS file picker
  dropZone.addEventListener('click', async () => {
    try {
      await invoke('trigger_file_dialog');
    } catch (err) {
      console.error('Failed to trigger file dialog:', err);
    }
  });

  // Setup Tauri drag & drop listeners
  if (window.__TAURI__) {
    const { listen } = window.__TAURI__.event;
    
    listen('tauri://drag-enter', () => {
      dragOverlay.classList.remove('hidden');
    });
    
    listen('tauri://drag-leave', () => {
      dragOverlay.classList.add('hidden');
    });
    
    listen('tauri://drag-drop', (event) => {
      dragOverlay.classList.add('hidden');
      const paths = event.payload.paths;
      if (paths && paths.length > 0) {
        handleFileDrop(paths);
      }
    });

    listen('upload-file-selected', (event) => {
      const path = event.payload;
      if (path) {
        handleFileDrop([path]);
      }
    });
  }

  // Initialize Auth Check
  init();
});
