<script>
    import LoadingBar from '../components/LoadingBar.svelte';
    import PostcodeInput from '../components/PostcodeInput.svelte';
    import VoteSuggestion from '../components/VoteSuggestion.svelte';
    import Sharing from '../components/Sharing.svelte';
    import InformationSection from '../components/InformationSection.svelte';
    import Footer from '../components/Footer.svelte';

    let loading = false;
    let suggestion = {};
</script>

<style>
    main {
        font-family: Roboto, -apple-system, BlinkMacSystemFont, Segoe UI, Oxygen,
            Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif;
        font-size: 16px;
        line-height: 1.5;
        color: #333;
        display: flex;
        flex-direction: column;
        max-width: 760px;
        background-color: white;
        padding: 0;
        margin: 0 auto;
        box-sizing: border-box;
        width: 100%;
        height: 100%;
    }
</style>

<svelte:head>
    <title>VoteABC: Anything But Conservative</title>
</svelte:head>

<main>
    <LoadingBar {loading} />
    <PostcodeInput bind:loading bind:suggestion />
    <VoteSuggestion {suggestion} />
    <Sharing {suggestion} />
    <InformationSection />
    <Footer />
</main>
