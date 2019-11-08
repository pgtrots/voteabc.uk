<script>
    import { format, formatDistanceToNow } from 'date-fns';

    export let text;
    export let date;
    export let urgency;
</script>

<style>
    div {
        width: calc(100% - 6em);
        margin: 2em;
        padding: 1em;
        text-align: center;
        border: 1px solid black;
    }

    div.urgent {
        background: #ffcc0055;
    }

    div.critical {
        background: #dc241f55;
        border: 2px solid red;
    }
</style>

<div class={urgency}>
    <span>
        <span class="text">
            <b>{text}:</b>
        </span>
        <span class="date">
            <b>{format(new Date(date), 'EEEE do MMMM')}</b>
            ({formatDistanceToNow(new Date(date), { addSuffix: true })})
        </span>
    </span>
</div>
