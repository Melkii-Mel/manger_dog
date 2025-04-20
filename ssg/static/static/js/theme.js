{
    function setupTheme() {
        const preferredTheme = localStorage.getItem('theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
        const currentTheme = preferredTheme || 'light';
        document.body.setAttribute('data-theme', currentTheme);

        if (toggle && !toggle.hasAttribute('data-theme-listener')) {
            toggle.setAttribute('data-theme-listener', 'true');
            toggle.addEventListener('click', () => {
                const current = document.body.getAttribute('data-theme');
                const next = current === 'dark' ? 'light' : 'dark';
                document.body.setAttribute('data-theme', next);
                localStorage.setItem('theme', next);
            });
        }
    }

    const toggle = document.currentScript.closest('.theme-toggle');
    window.addEventListener('DOMContentLoaded', setupTheme);
    window.addEventListener('pageshow', setupTheme);
}