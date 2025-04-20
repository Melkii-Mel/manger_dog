{
    const label = document.currentScript.parentElement;
    /**
     * @type {HTMLInputElement}
     */
    const input = label.querySelector(".form-input-input")
    const update_data_empty = () => label.setAttribute("data-empty", (input.value === "" && input.placeholder === "").toString());
    update_data_empty()
    input.addEventListener("input", update_data_empty);
}