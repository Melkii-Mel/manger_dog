function get_cookie_by_name(name) {
    const cookies = document.cookie.split(';');
    for (const cookie of cookies) {
        const [key, value] = cookie.split("=").map(c => c.trim());
        if (key === name) {
            return value;
        }
    }
    return undefined;
}

function authorized() {
    // HACK: Hardcoded value
    return get_cookie_by_name("refresh_token_dummy") !== undefined;
}

function go_to_app_if_authorized() {
    if (authorized()) {
        window.location.href = "/app";
    }
}