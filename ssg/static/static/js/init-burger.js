{
    const burger_items = document.currentScript.closest(".burger-items")
    document.addEventListener("DOMContentLoaded", () => {
        const burger_button = burger_items.querySelector(".burger-button");
        const burger_menu = burger_items.querySelector(".burger-menu");
        const burger_menu_close_button = burger_items.querySelector(".burger-menu-close-button");
        document.body.appendChild(burger_menu);
        burger_items.parentElement.appendChild(burger_button);
        burger_button.addEventListener("click", () => {
            burger_menu.classList.toggle("burger-menu-open");
        })
        burger_menu_close_button.addEventListener("click", () => {
            burger_menu.classList.remove("burger-menu-open");
        })
    })
}