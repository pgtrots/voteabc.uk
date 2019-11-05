<script>
    import RotatingArrow from './RotatingArrow.svelte';

    import { slide } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';

    export let question;
    let isOpen = false;
</script>

<style>
    hr {
        border: 0;
        height: 1px;
        background-image: linear-gradient(
            to right,
            rgba(0, 0, 0, 0),
            rgba(0, 0, 0, 0.25),
            rgba(0, 0, 0, 0)
        );
    }

    span.question-title {
        cursor: pointer;
    }

    h3 {
        font-size: 1em;
        padding: 0.75em 0em 0.75em 0em;
    }
</style>

<hr />
<div class="question">
    <span
        class="question-title"
        on:click={() => {
            isOpen = !isOpen;
        }}>
        <h3>
            <RotatingArrow rotate={isOpen} />
            {@html question.title}
        </h3>
    </span>
    {#if isOpen}
        <div
            class="question-body"
            transition:slide={{ duration: 400, easing: quintOut }}>
            {@html question.body}
        </div>
    {/if}
</div>
