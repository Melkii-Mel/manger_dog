const common_validators = {
    not_empty: (/** @type {HTMLInputElement} */ e) => {
        if (e.value.trim() === '') {
            return 'StringIsEmpty';
        }
    },
    email: (/** @type {HTMLInputElement} */ e) => {
        const email = e.value;

        if (email.trim() === '') {
            return 'StringIsEmpty';
        }

        const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
        if (!emailRegex.test(email)) {
            return 'EmailFormatInvalid';
        }

        return undefined;
    },

    password: (/** @type {HTMLInputElement} */e, /** @type {number} */ strictness) => {
        const password = e.value;

        if (password.trim() === '') {
            return 'StringIsEmpty';
        }
        if (password.length < (strictness === 0 ? 6 : strictness === 1 ? 8 : 12)) {
            return 'PasswordTooShort';
        }
        if (password.length > 64) {
            return 'PasswordTooLong';
        }
        if (/\s/.test(password)) {
            return 'PasswordMustNotContainSpaces';
        }
        if (!/^[a-zA-Z0-9@#!$%^&*()_+\-=<>?{}\[\]|~]+$/.test(password)) {
            return 'PasswordContainsInvalidCharacters';
        }
        if (!/\d/.test(password)) {
            return 'PasswordMustContainDigit';
        }
        if (strictness === 0) {
            return;
        }
        if (!/[A-Z]/.test(password)) {
            return 'PasswordMustContainUppercase';
        }
        if (!/[a-z]/.test(password)) {
            return 'PasswordMustContainLowercase';
        }
        if (strictness === 1) {
            return;
        }
        if (!/[^\w\s:]/.test(password)) {
            return 'PasswordMustContainSpecial';
        }
    }
};
