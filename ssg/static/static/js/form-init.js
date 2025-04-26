/**
 *
 * @param {HTMLFormElement} form
 * @param {Object.<string, function(RadioNodeList|HTMLInputElement|HTMLTextAreaElement|Element):string|undefined>} validations
 * @param {string} address
 * @param {string} method
 */
const form_init = (form, validations, address, method = "POST") => {
    const validators = {};
    const error_elements = {};
    const submit_button = form.querySelector('[type="submit"]');
    for (const element of form.elements) {
        error_elements[element.name] = element.parentElement.parentElement.querySelector(".form-input-error");
    }
    Object.entries(validations).forEach(([name, fn]) => {
        const element = form.elements[name];
        validators[name] = () => {
            const error = fn(element);
            const valid = error === null || error === undefined;
            error_elements[name].innerText = error_messages[error] ?? error ?? "";
            element.setAttribute("data-erroneous", (!valid).toString());
            submit_button.disabled = Array.from(form.elements).some(
                e => e.getAttribute("data-erroneous") === "true"
            );
            return valid;
        }
        element.addEventListener("input", () => {
            validators[name]();
        });
        element.addEventListener("focusout", () => {
            validators[name]();
        })
    })
    form.addEventListener("submit", async e => {
        e.preventDefault();
        let any_errors = false;
        Object.entries(validations).forEach(([name, _]) => {
            if (!validators[name]()) {
                any_errors = true;
            }
        })
        if (any_errors) {
            return
        }

        const form_data = new FormData(form);
        const body = {};
        form_data.entries().forEach(e => body[e[0]] = e[1]);
        console.log(JSON.stringify(body));
        const response = await fetch(address, {
            method: method,
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(body),
        });
        const contentType = response.headers.get('Content-Type');
        let json;
        if (contentType && contentType.includes('application/json')) {
            json = await response.json();
        } else {
            console.log(await response.text());
            json = {};
        }
        switch (json["Err"]) {
            case undefined: {
                window.location.href = "/app/";
                break;
            }
            case "EmailTaken": {
                error_elements["email"].innerText = error_messages["EmailTaken"];
                form.elements["email"].setAttribute("data-erroneous", "true");
                break;
            }
            case "InvalidCredentials": {
                error_elements["password"].innerText = error_messages["InvalidCredentials"];
                break;
            }
            default: {
                const error = json["Err"];
                Object.keys(error).forEach(key => {
                    let e = error[key];
                    while (typeof e === 'object' && e !== null) {
                        const keys = Object.keys(e);
                        e = e[keys[0]];
                    }
                    error_elements[key].innerText = error_messages[e] ?? "";
                })
                break;
            }
        }
    })
}