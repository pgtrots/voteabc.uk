<script>
    import Question from './Question.svelte';

    function daysFromNow(isodate) {
        let days = Math.floor(
            (new Date(isodate) - new Date()) / (1000 * 60 * 60 * 24)
        );
        if (days > 1) return days.toString() + ' days away';
        if (days == 1) return 'TOMORROW! REMEMBER TO VOTE!';
        if (days == 0) return 'TODAY!! GO AND VOTE!';
        return 'Already happened... good luck everyone.';
    }

    const FAQs = [
        {
            title: 'Why should I not vote for the Conservatives?',
            body: `
            <p>In short, because they have consistently shown themselves to be cruel, self-interested individuals who <b>do not</b> have the interests of the British people at heart, and whose only real talent is in convincing them otherwise.</p>
            `,
        },
        {
            title: "Why can't I just vote for who <i>I</i> want?",
            body: `
            <p>Unfortunately, the British electoral system results in the majority of voters having no direct representation in Parliament. This is because, under the "First Past the Post" system, votes which are <i>not</i> cast for the winning candidate are essentially entirely ignored.</p>
            <p>This means that votes for parties other than the closest competitor for the Conservatives do nothing to help reduce the chance of a Conservative victory. This is why we advocate voting for the Conservatives closest competitor regardless of your other party loyalties. This is colloquially known as "Tactical Voting". If you do live a constituency where there is very little risk of the Conservatives winning (a margin of 20% or more) then you can safely vote freely.</p>
            <p>For a clear and concise explanation on the subject, you can <a href="https://www.youtube.com/watch?v=r9rGX91rq5I" target="_blank">check out this 5 minute video</a>, and for a comprehensive breakdown of the UK electoral system's failings there is <a href="https://www.electoral-reform.org.uk/latest-news-and-research/publications/the-2017-general-election-report/" target="_blank">this article</a>.</p>
            `,
        },
        {
            title: 'But I voted Leave so who should I vote for?',
            body: `
            <p>If you are still set on the UK leaving the EU, then please vote for <a href="https://www.thebrexitparty.org/" target="_blank">the Brexit Party</a>. They are fighting for a real break from the EU and will not settle for any sort of "deal". Honestly, this is not about leave/remain, this is about voting for <i>anything</i> except the Conservatives.</p>
            `,
        },
        {
            title: "I'm just tired of Brexit and want the country to move on.",
            body: `
            <p>If this is your point of view then you absolutely should <b>not</b> vote for the Conservatives. The only thing that allowing a Conservative majority to pass Boris Johnson's deal guarantees is literally <i>decades</i> more uncertainty and strife as the UK seeks to renogotiate all the trade agreements it will have lost.</p>
            <p>Leaving the EU is not the end of Brexit, it is only the end of the beginning.</p>
            `,
        },
    ];
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
    <p>
        <b>
            Polling day: Thursday, 12th December ({daysFromNow('2019-12-12')})
        </b>
    </p>
    <div class="emergency">
        <a href="https://www.gov.uk/register-to-vote" target="_blank">
            Register to vote
        </a>
    </div>
    <p>
        <b>
            Deadline to register: Tuesday, 26th November ({daysFromNow('2019-11-26')})
        </b>
    </p>
    {#each FAQs as question}
        <Question {question} />
    {/each}
</section>
