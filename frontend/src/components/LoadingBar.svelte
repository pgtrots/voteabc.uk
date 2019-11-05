<script>
    export let loading;
    let nrects = 4;
</script>

<style>
    div {
        height: 1.5em;
    }

    svg {
        padding: 0;
        height: 100%;
        width: 100%;
        display: block;
    }

    rect:nth-child(4n + 1) {
        fill: #67b437;
    }
    rect:nth-child(4n + 2) {
        fill: #ffcc00;
    }
    rect:nth-child(4n + 3) {
        fill: #dc241f;
    }
    rect:nth-child(4n + 4) {
        fill: #aaa;
    }

    @keyframes flash1 {
        0% {
            opacity: 1;
        }
        15% {
            opacity: 0.25;
        }
        30% {
            opacity: 1;
        }
    }

    @keyframes flash2 {
        25% {
            opacity: 1;
        }
        40% {
            opacity: 0.25;
        }
        55% {
            opacity: 1;
        }
    }

    @keyframes flash3 {
        50% {
            opacity: 1;
        }
        65% {
            opacity: 0.25;
        }
        80% {
            opacity: 1;
        }
    }

    @keyframes flash4 {
        75% {
            opacity: 1;
        }
        90% {
            opacity: 0.25;
        }
        100% {
            opacity: 1;
        }
    }

    rect.loading:nth-child(4n + 1) {
        animation: flash1 0.5s linear 0s infinite normal;
    }

    rect.loading:nth-child(4n + 2) {
        animation: flash2 0.5s linear 0s infinite normal;
    }

    rect.loading:nth-child(4n + 3) {
        animation: flash3 0.5s linear 0s infinite normal;
    }

    rect.loading:nth-child(4n + 4) {
        animation: flash4 0.5s linear 0s infinite normal;
    }
</style>

<div>
    <svg
        viewBox="0 0 100 100"
        preserveAspectRatio="none"
        xmlns="http://www.w3.org/2000/svg">
        {#each Array.from({ length: nrects }) as _, i}
            <rect
                width="{100 / nrects}%"
                height="100%"
                x="{(100 * i) / nrects}%"
                class:loading />
        {/each}
    </svg>
</div>
