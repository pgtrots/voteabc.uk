<script>
    import * as colours from '../styles/colours.js';
    export let suggestion;

    function validateSuggestion(suggestion) {
        return suggestion.party !== undefined;
        return (
            suggestion.postcode !== undefined &&
            suggestion.constituency !== undefined &&
            suggestion.party !== undefined &&
            suggestion.majority !== undefined &&
            suggestion.majorityPercent !== undefined &&
            true
        );
    }

    const PARTY_MAP = {
        'Democratic Unionist Party': {
            type: 'svg',
            src: '/logos/Democratic_Unionist_Party_logo_notext.svg',
            text: 'the Democratic Unionist Party',
            color: colours.abcRed,
        },
        'Green Party': {
            type: 'svg',
            src: '/logos/Green_Party_of_England_and_Wales_logo_notext.svg',
            text: 'the Green Party',
            color: colours.abcGreen,
        },
        Labour: {
            type: 'svg',
            src: '/logos/Labour_Party_logo_notext.svg',
            text: 'Labour',
            color: colours.abcRed,
        },
        'Liberal Democrats': {
            type: 'svg',
            src: '/logos/Liberal_Democrats_logo_notext.svg',
            text: 'the Liberal Democrats',
            color: colours.abcYellow,
        },
        'Plaid Cymru': {
            type: 'svg',
            src: '/logos/Plaid_Cymru_logo_notext.svg',
            text: 'Plaid Cymru',
            color: colours.abcGreen,
        },
        'Scottish National Party': {
            type: 'svg',
            src: '/logos/Scottish_National_Party_logo_notext.svg',
            text: 'the Scottish National Party',
            color: colours.abcYellow,
        },
        'Sinn Fein': {
            type: 'svg',
            src: '/logos/Sinn_Fein_logo_notext.svg',
            text: 'Sinn Féin',
            color: colours.abcGreen,
        },
        'Claire Wright': {
            type: 'jpg',
            src: '/images/Claire_Wright.jpg',
            text: 'Claire Wright',
            color: colours.abcGrey,
        },
        'Sylvia Hermon': {
            type: 'jpg',
            src: '/images/Silvia_Hermon.jpg',
            text: 'Sylvia Hermon',
            color: colours.abcGrey,
        },
        'Dominic Grieve': {
            type: 'jpg',
            src: '/images/Dominic_Grieve.jpg',
            text: 'Dominic Grieve',
            color: colours.abcGrey,
        },
    };

    let image;
    $: image = PARTY_MAP[suggestion.party];

    function randomChoice(array) {
        return array[Math.floor(Math.random() * array.length)];
    }

    function motivationalText(majorityPercent) {
        majorityPercent = parseFloat(majorityPercent);

        if (majorityPercent > 0.15) {
            return randomChoice([
                'A comfortable lead but please make sure you register and get out there on polling day regardless.',
            ]);
        }
        if (majorityPercent > 0.05) {
            return randomChoice([
                'It will be very important to go and vote on election day to maintain the lead.',
            ]);
        }
        if (majorityPercent > 0.01) {
            return randomChoice([
                'This is only a very slim lead, please be sure to go and vote!',
            ]);
        }
        if (majorityPercent > -0.01) {
            return randomChoice([
                "We're so close to winning this! Please be sure to go and vote. Tell your friends.",
            ]);
        }
        if (majorityPercent > -0.05) {
            return randomChoice([
                'This is well within reach! Forward this on to all you can, make sure they are registered to vote and bring them with you on polling day.',
                'A significant gap, but it can still be overcome if you make sure all your friends and family are registered and vote on the day!',
            ]);
        }
        if (majorityPercent > -0.1) {
            return randomChoice([
                "This will require some serious work to kick the Tories out, but don't despair, it can still be done! Tell everyone you can to get out and vote!",
            ]);
        }
        return randomChoice([
            'This constituency may be a lost cause, but please still vote, and tell your friends who live elsewhere to do the same. Unfortunately your vote has and will continue to be totally unrepresented in parliament.',
        ]);
    }
</script>

<style>
    section {
        text-align: center;
    }

    svg,
    img {
        margin: 0 1em 1em 1em;
        box-sizing: border-box;
        width: 90%;
        max-width: 40em;
    }

    p {
        text-align: justify;
        margin: 0.5em 1.5em 0.5em 1.5em;
    }
</style>

<section>
    {#if !validateSuggestion(suggestion)}
        <img src="/logos/vote_abc.svg" alt="Anything But Conservative" />
    {:else}
        <img src={image.src} alt={suggestion.party} />
        <h1>
            Vote for
            <b>{image.text}</b>
        </h1>
        <p>
            In 2017, in your constituency ({suggestion.constituency}), the
            Conservatives {parseInt(suggestion.majority) > 0 ? 'were defeated' : 'won'}
            by {Math.abs(parseInt(suggestion.majority))} votes ({Math.abs(suggestion.majorityPercent * 100).toFixed(1)}%
            of those who voted).
        </p>
        <p>{motivationalText(suggestion.majorityPercent)}</p>
    {/if}
</section>
