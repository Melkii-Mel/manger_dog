{
    if (languages === undefined) {
        throw new Error("List of languages must be defined in the document");
    }
    const select = document.currentScript.closest(".language-select");
    window.addEventListener("pageshow", () => {
        select.value = languages.current;
    });
    select.addEventListener("change", function () {
        const selectedLang = select.value;
        localStorage.setItem("lang", selectedLang);
        const pathParts = window.location.pathname.split("/");
        if (selectedLang === languages.default) {
            pathParts.splice(1, 1);
            if (pathParts.length === 1) {
                pathParts.push("");
            }
        } else {
            if (languages.includes(pathParts[1])) {
                pathParts[1] = selectedLang;
            } else {
                pathParts.splice(0, 1, selectedLang);
            }
        }
        window.location.href = pathParts.join("/");
    });
}