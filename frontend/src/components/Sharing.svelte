<script>
    export let suggestion;

    import copy from 'clipboard-copy';

    function handleCopy() {
        copy('https://voteabc.uk');
        alert('Copied link to clipboard');
    }

    let title = 'VoteABC: Anything But Conservative, a tactical voting app.';
    let body =
        suggestion.constituency !== undefined
            ? `I just looked up who I should vote for in ${suggestion.constituency} in order to get rid of the Conservative Government. Visit VoteABC.uk to find out who you should vote for too! #VoteAnythingButConservative #ToriesOut`
            : 'Are you worried the Conservatives might win the next election, but you’re not sure who to vote for to vote to stop that?  Check out VoteABC.uk and we’ll help you decide! #VoteAnythingButConservative #ToriesOut';
</script>

<style>
    section {
        width: calc(100% - 4em);
        margin: 0 2em;
        margin-top: 1em;
        box-sizing: border-box;
        text-align: right;
    }

    span {
    }

    a {
        padding: 0 0.5em;
    }

    svg {
        height: 1.5em;
        fill: #333;
    }
</style>

<section>
    <span>
        <a href="" on:click|preventDefault={handleCopy}>
            <svg
                viewBox="0 0 1024.0008544921875 1024.1280517578125"
                aria-labelledby="copy to clipboard"
                id="si-flat-export">
                <path
                    d="M896 896.128H128v-768h320v-128L89.024.064C39.872.064 0 40
                    0 89.152v845.952c0 49.152 39.872 89.024 89.024
                    89.024h845.952c49.152 0 89.024-39.872
                    89.024-89.024V576.128H896v320zM1024
                    64c0-14.656-6.08-27.52-14.72-38.272-1.344-1.728-2.048-3.712-3.584-5.312-.192-.128-.256-.384-.384-.576-.384-.32-.448-.832-.832-1.216-4.096-4.096-9.152-6.528-13.952-9.28-2.112-1.216-3.84-3.008-6.08-3.968C975.744
                    1.6 966.528 0 957.184.256c-.128
                    0-.256-.064-.384-.064H643.777C606.785.128 576.769 28.736
                    576.769 64c0 35.2 30.08 63.808 67.136 63.808h161.216l-402.56
                    403.328c-24.832 24.768-24.832 64.768 0 89.472 24.832 24.768
                    65.024 24.768 89.792 0l403.968-403.52v163.2c0 37.056 28.608
                    67.072 63.872 67.072s63.808-30.016
                    63.808-67.072V67.264c0-.64-.32-1.152-.32-1.728
                    0-.512.32-1.024.32-1.536z" />
            </svg>
        </a>
        <a href={`mailto:?subject=${title}&body=${body}`}>
            <svg
                viewBox="0 0 514.8550415039062 347.53997802734375"
                aria-labelledby="email link"
                id="si-zocial-email">
                <path
                    d="M0 316.758V30.782c0-.33.496-3.475 1.49-9.433l168.308
                    143.98L1.986
                    326.688c-1.323-4.634-1.985-7.944-1.985-9.93zM22.342
                    1.49C24.659.497 27.472 0 30.782 0h453.29c2.98 0 5.958.497
                    8.937 1.49L324.204 145.968l-22.342 17.873-44.187
                    36.244-44.187-36.244-22.342-17.873zm.496 344.56L192.14
                    183.7l65.536 53.123 65.536-53.124L492.513
                    346.05c-2.648.994-5.461 1.49-8.44 1.49H30.783c-2.649
                    0-5.297-.496-7.945-1.49zm322.716-180.719L513.366 21.35c.993
                    2.98 1.489 6.124 1.489 9.434V316.76c0 2.978-.496 6.288-1.49
                    9.93z" />
            </svg>
        </a>
        <a
            href="https://www.facebook.com/sharer/sharer.php?u=https://voteabc.uk">
            <svg
                viewBox="0 0 275.45599365234375 512"
                aria-labelledby="share to facebook"
                id="si-zocial-facebook">
                <path
                    d="M0 274.944v-97.776h78.865V128c0-35.504 11.504-65.697
                    34.544-90.624C136.449 12.463 164.369 0 197.12
                    0h78.336v97.792H197.12c-5.105 0-9.712 2.816-13.808
                    8.448-4.097 5.632-6.144 12.544-6.144
                    20.735v50.177h98.288v97.775h-98.304V512H78.847V274.943H0z" />
            </svg>
        </a>
        <a
            href={`https://twitter.com/intent/tweet?text=${body}&url=https://voteabc.uk`}>
            <svg
                viewBox="0 0 640.0180053710938 520.3389892578125"
                aria-labelledby="share on twitter"
                id="si-zocial-twitter">
                <path
                    d="M0 461.54c10.42 1.014 20.826 1.548 31.22 1.548 61.048 0
                    115.528-18.732
                    163.387-56.17-28.424-.352-53.933-9.041-76.477-26.043-22.57-16.99-37.984-38.675-46.323-65.056
                    6.933 1.418 15.102 2.095 24.456 2.095 12.15 0 23.766-1.575
                    34.862-4.684-30.517-5.866-55.766-20.891-75.709-44.996-19.955-24.13-29.919-51.969-29.919-83.527v-1.574c18.395
                    10.42 38.31 15.805 59.826
                    16.13-18.016-11.798-32.338-27.304-42.915-46.57-10.575-19.24-15.87-40.13-15.87-62.675
                    0-23.597 6.088-45.607 18.212-66.095 32.6 40.586 72.418
                    72.938 119.431 97.055 47 24.092 97.368 37.53 151.158
                    40.327-2.432-11.448-3.655-21.516-3.655-30.18 0-36.085
                    12.84-66.954 38.505-92.62C375.868 12.839 406.893 0 443.342
                    0c37.79 0 69.7 13.88 95.73 41.64 30.167-6.257 57.926-17.015
                    83.255-32.261-9.718 31.558-28.815 55.845-57.238 72.847
                    25.327-3.109 50.304-10.055 74.929-20.813-16.651
                    26.017-38.336 48.742-65.056 68.151v17.198c0 34.992-5.125
                    70.128-15.35 105.355-10.211 35.214-25.848 68.853-46.83
                    100.972-20.996 32.065-46.05 60.619-75.189 85.569-29.126
                    24.977-64.08 44.853-104.849 59.592-40.755 14.752-84.555
                    22.089-131.398
                    22.089-72.483-.014-139.606-19.605-201.345-58.8z" />
            </svg>
        </a>
    </span>
</section>
