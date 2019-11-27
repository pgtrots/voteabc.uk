<script>
    import Question from './Question.svelte';
    import Deadline from './Deadline.svelte';
    import FAQs from '../data/faqs.js';

    function daysFromNow(isodate) {
        let days = Math.floor(
            (new Date(isodate) - new Date()) / (1000 * 60 * 60 * 24)
        );
        if (days > 1) return days.toString() + ' days away';
        if (days == 1) return 'TOMORROW! REMEMBER TO VOTE!';
        if (days == 0) return 'TODAY!! GO AND VOTE!';
        return 'Already happened... good luck everyone.';
    }
</script>

<style>
    section {
        margin: 1em;
        text-align: justify;
    }

    hr {
        border: 0;
        height: 0;
        border-top: 1px solid rgba(0, 0, 0, 0.1);
        border-bottom: 1px solid rgba(255, 255, 255, 0.3);
    }

    div.emergency {
        cursor: pointer;
        font-size: 1.5em;
        line-height: 2em;
        background-color: #370059;
        box-shadow: inset 0em 1em #4b007a, 0 0 0.25em black;
        text-align: center;
    }

    @media only screen and (min-width: 700px) {
        div.emergency {
            font-size: 3em;
        }
    }

    div.emergency a {
        padding: 0px 2.25em;
        font-weight: 600;
        color: #fff;
        text-transform: uppercase;
        text-decoration: none;
        text-shadow: 0.1em 0.1em 0px rgba(0, 0, 0, 0.25);
    }
</style>

<section>
    <hr />
    <Deadline text="Polling day" date="2019-12-12" urgency="urgent" />
    {#each FAQs as question}
        <Question {question} />
    {/each}
</section>
