// for shortcodes/tabs.html
function scSwitchTab(btn, id) {
    const group = btn.closest('.sc-tabs');
    group.querySelectorAll('.sc-tab-btn').forEach(b => b.classList.remove('active'));
    group.querySelectorAll('.sc-tab-pane').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    document.getElementById(id).classList.add('active');
}
// for shortcodes/tabs.html end