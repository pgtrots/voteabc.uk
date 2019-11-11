<script>
    import partyInfo from '../data/party-info.js';
    import motivationalText from '../data/motivational-text.js';

    export let suggestion;

    function validateSuggestion(suggestion) {
        return (
            suggestion.postcode !== undefined &&
            suggestion.constituency !== undefined &&
            suggestion.party !== undefined &&
            suggestion.source !== undefined &&
            suggestion.majority !== undefined &&
            suggestion.majorityPercent !== undefined &&
            true
        );
    }

    let dataSource;
    $: switch (suggestion.source) {
        case '2017 Election': {
            dataSource = {
                href: `https://en.wikipedia.org/wiki/${suggestion.constituency}_(UK_Parliament_constituency)`,
                body: '2017 Election closest competitor',
            };
            break;
        }
        case 'Best For Britain': {
            dataSource = {
                href: 'https://getvoting.org',
                body: 'Best for Britain MRP',
            };
            break;
        }
        default: {
            dataSource = undefined;
            break;
        }
    }
</script>

<style>
    section {
        text-align: center;
    }

    img {
        margin: 0 1em 1em 1em;
        box-sizing: border-box;
        width: 90%;
        max-width: 40em;
    }

    img.abc {
        margin-bottom: 4em;
    }

    p {
        text-align: justify;
        margin: 0.5em 1.5em 0.5em 1.5em;
    }
</style>

<section>
    {#if !validateSuggestion(suggestion)}
        <img
            class="abc"
            src="/logos/vote_abc_thorns.svg"
            alt="Anything But Conservative" />
    {:else}
        <img src={partyInfo[suggestion.party].src} alt={suggestion.party} />
        <h1>
            Vote for
            <b>{partyInfo[suggestion.party].text}</b>
        </h1>
        <p>
            In 2017, in your constituency ({suggestion.constituency}), the
            Conservatives {parseInt(suggestion.majority) > 0 ? 'were defeated' : 'won'}
            by {Math.abs(parseInt(suggestion.majority))} votes ({Math.abs(suggestion.majorityPercent * 100).toFixed(1)}%
            of those who voted).
        </p>
        <p>
            {@html motivationalText(suggestion)}
        </p>
        {#if dataSource}
            <p>
                Data Source:
                <a href={dataSource.href}>{dataSource.body}</a>
            </p>
        {/if}
    {/if}
</section>
