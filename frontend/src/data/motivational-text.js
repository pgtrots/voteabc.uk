function randomChoice(array) {
    return array[Math.floor(Math.random() * array.length)];
}

export default function motivationalText(suggestion) {
    let majorityPercent = parseFloat(suggestion.majorityPercent);

    if (suggestion.constituency === 'Uxbridge and South Ruislip') {
        return 'This is Boris Johnson\'s constituency, you simply MUST go and vote against him! Tell literally everyone you can, the seat can be destabilised enough to oust him, which would be wonderful.';
    }

    if (suggestion.constituency === 'Chorley') {
        return 'It literally doesn\'t matter how you\'d like to vote - your MP is the speaker of the House of Commons and therefore is traditionally unopposed. This means your vote has a 0% chance of counting possibly for the next 10 years. You can honestly not bother on election day, unless you wish to vote to protest the point.';
    }

    if (majorityPercent > 0.15) {
        return randomChoice([
            'A comfortable lead but please make sure you register and get out there on polling day regardless.',
            'This party had a comfortable lead in the last election, but this is not a guaranteed win for this election - please make sure you register to vote and cast your ballot on polling day.',
        ]);
    }
    if (majorityPercent > 0.05) {
        return randomChoice([
            'It will be very important to go and vote on election day to maintain the lead.',
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
            'We\'re so close to winning this! Please be sure to go and vote. Tell your friends.',
            'We\'re so close to winning this! Please be sure to vote, and don’t forget to tell your friends.',
        ]);
    }
    if (majorityPercent > -0.05) {
        return randomChoice([
            'This is well within reach! Forward this on to all you can, make sure they are registered to vote and bring them with you on polling day.',
            'This is well within reach! Forward this on to all you can, make sure they are registered to vote and bring them with you on polling day.',
        ]);
    }
    if (majorityPercent > -0.1) {
        return randomChoice([
            'A significant gap, but it can still be overcome if you make sure all your friends and family are registered and vote on the day!',
            'A significant gap, but it can still be overcome if you make sure all your friends and family are registered and vote on the day!',
        ]);
    }
    if (majorityPercent > -0.2) {
        return randomChoice([
            'This will require some serious work to kick the Tories out, but don\'t despair, it can still be done! Tell everyone you can to get out and vote!',
            'This will require some serious work to kick the Tories out, but don\'t despair – it can still be done! Tell everyone you can to get out and vote!',
        ]);
    }
    return randomChoice([
        'This constituency may be a lost cause, but please still vote, and tell your friends who live elsewhere to do the same. Unfortunately your vote has and will continue to be totally unrepresented in parliament.',
        'This constituency may feel like a lost cause, but please still vote, and tell your friends – wherever they live – to do the same. It may seem like your vote will continue to be totally unrepresented in parliament, but all of these elections rest on the votes of single people.',
    ]);
}
