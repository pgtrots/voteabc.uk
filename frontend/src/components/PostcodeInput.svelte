<script>
    export let loading;
    export let suggestion;

    const DEFAULT_INVALID_MESSAGE = 'Not a valid postcode.';
    const API_URL = '/api';

    let postcode = '';
    let invalid = false;
    let invalidMessage = DEFAULT_INVALID_MESSAGE;

    // from https://stackoverflow.com/a/7259020
    const POSTCODE_REGEX = /^(([gG][iI][rR] {0,}0[aA]{2})|((([a-pr-uwyzA-PR-UWYZ][a-hk-yA-HK-Y]?[0-9][0-9]?)|(([a-pr-uwyzA-PR-UWYZ][0-9][a-hjkstuwA-HJKSTUW])|([a-pr-uwyzA-PR-UWYZ][a-hk-yA-HK-Y][0-9][abehmnprv-yABEHMNPRV-Y]))) {0,}[0-9][abd-hjlnp-uw-zABD-HJLNP-UW-Z]{2}))$/;

    async function handleSubmit() {
        suggestion = {};

        if (!postcode.match(POSTCODE_REGEX)) {
            invalid = true;
            invalidMessage = DEFAULT_INVALID_MESSAGE;
            return;
        } else {
            invalid = false;
        }

        const url =
            API_URL +
            '/' +
            postcode
                .trim()
                .replace(' ', '')
                .toUpperCase();

        loading = true;
        try {
            const res = await fetch(url);
            if (!res.ok) {
                if (res.status === 404) {
                    invalid = true;
                    invalidMessage = DEFAULT_INVALID_MESSAGE;
                    loading = false;
                    return;
                } else {
                    throw new Error(res.statusText);
                }
            }
            suggestion = await res.json();
        } catch (e) {
            console.error(e);
            invalid = true;
            invalidMessage = 'An error occurred, please try again later...';
        }
        loading = false;
    }
</script>

<style>
    section {
        font-size: 1.5em;
        padding-top: 1em;
        width: 100%;
        text-align: center;
    }

    @media only screen and (min-width: 700px) {
        section {
            font-size: 3em;
        }
    }

    form {
        display: flex;
        flex-direction: row;
        justify-content: center;
        font-size: inherit;
        margin-top: 1em;
        margin-bottom: 2em;
    }

    input {
        width: 90%;
        height: 2em;
        text-align: center;
        font-size: inherit;
        display: block;
        border: 2px solid #ccc;
        border-radius: 0.2em;
        box-sizing: border-box;
    }

    input.invalid {
        border: 2px solid #dc241f;
    }

    span#invalid-label {
        font-size: 0.5em;
        height: 1em;
        margin-top: -2em;
        margin-bottom: 1em;
        color: #dc241f;
        display: none;
    }

    span#invalid-label.invalid {
        display: block;
    }

    button {
        height: 2em;
        width: 2em;
        margin-left: -2em;
        padding: 0;
        font-size: inherit;
        display: block;
        border-radius: 0 0.16em 0.16em 0;
        border: none;
        background-color: #67b437;
        box-sizing: border-box;
        cursor: pointer;
    }

    button svg {
        fill: #eee;
        margin: 0.3em;
    }
</style>

<section>
    <form on:submit|preventDefault={handleSubmit}>
        <input
            name="postcode"
            type="text"
            placeholder="Enter your postcode"
            aria-label="Postcode"
            class:invalid
            bind:value={postcode} />
        <button type="submit">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1515 1515">
                <path
                    d="M363 1387l91-91-235-235-91
                    91v107h128v128h107zm523-928q0-22-22-22-10 0-17 7L305 986q-7
                    7-7 17 0 22 22 22 10 0 17-7l542-542q7-7 7-17zm-54-192l416
                    416-832 832H0v-416zm683 96q0 53-37 90l-166 166-416-416
                    166-165q36-38 90-38 53 0 91 38l235 234q37 39 37 91z" />
            </svg>
        </button>
    </form>
    <span id="invalid-label" class:invalid>{invalidMessage}</span>
</section>
