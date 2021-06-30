/* eslint-disable quotes */
import React from 'react';

import Iframe from 'components/iframe/iframe';
import { ResponsiveEmbed } from 'react-bootstrap';

import Planet from 'assets/images/planet.png';
import Car from 'assets/images/car.png';
import WHT from 'assets/images/wht.png';
import GenieOfLight from 'assets/images/genieoflight.png';
import TrolleyFolly from 'assets/images/trolleyfolly.png';
import Fractal from 'assets/images/fractal.png';
import VRExperiment from 'assets/images/vr_experiment.png';

const projects = [
    {
        id: 'pseudo-haptic-drift-correction',
        title: 'Pseudo Haptic Drift Correction in VR',
        date: 'May 2021',
        description: `As part of my degree I did a project over 6 months. The abstract of my project is as follows.

        Virtual reality is about stimulating the senses to create the illusion that the user exists in a simulated world. This is commonly achieved through the visual and auditory senses. It is also possible to stimulate the users sense of touch using haptics. Haptics are not widely available so we looked at pseudo haptics which is the method of creating the illusion of haptic stimulation using other senses. A recently proposed pseudo haptic mass illusion discussed how to create the illusion of objects having mass without using haptics. This project is about refining this pseudo haptic mass illusion. The technique involves creating a drift between the physical and real controller
        positions / rotations. We look at different techniques to correct this pseudo haptic drift whilst maintaining the embodiment illusion. In the project we implemented the pseudo haptic mass technique in a virtual reality application. We designed and implemented 8 techniques to correct the pseudo haptic drift. We ran a user study to compare the most effective correction method to realign the pseudo haptic drift. We find that our results indicate some techniques are better at correcting the drift than others whilst maintaining the embodiment illusion.
        `,
        image: VRExperiment,
        tags: ['VR', 'C#', 'Unity'],
    },
    {
        id: 'video-labeller',
        title: 'Video Labeller',
        date: 'July 2020',
        description: `My grandfather had a cine film camera and his films were digitised however they are not in order.
        I wrote a program that takes videos divided into scenes and allows the user to label and tag the videos.
        The program also allows the user to search for tags and play the selected videos in sequence.
        This is useful because relatives often only want to see videos that are relevant to them and sorting through 14 hours of video takes too long.
        The program uses FFmpeg to convert the videos into short clips.`,
        tags: ['C#'],
    },
    {
        id: 'l-systems',
        title: 'L-Systems',
        date: 'July 2020',
        description: `I was interested in L-systems which is a way of describing fractal patterns.
        It works by having a start string e.g. "X".
        Then having rules such as X -> XAX and A -> AB would result in the following:
        n = 0: X

        n = 1: XAX

        n = 2: XAXABXAX

        n = 3: XAXABXAXABBXAXABXAX

        n = 4: XAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAX

        n = 5: XAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAXABBBBXAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAX

        ...

        Then by creating rules based on the string for example forward, turn left and turn right we can draw images.`,
        image: Fractal,
        link: 'https://en.wikipedia.org/wiki/L-system',
        linkText: 'L-Systems Wikipedia',
        tags: ['Python'],
    },
    {
        id: 'trolley-folly',
        title: '48 Hour Game Jam: Trolley Folly',
        date: 'July 2020',
        description: `A friend and I participated in an online game jam where we made a game in 48 hours based on a theme.
        The theme was announced as Out of Control. Sam and I created a game where the player is trying to control and out of control shopping trolley as you speed around a supermarket collecting as many items as possible.
        The more items you stack in your cart the easier it is for items to fall off. The player has to balance the items, avoid crashing into things and take the items to the checkout.`,
        image: TrolleyFolly,
        link: 'https://wilkoco.itch.io/trolley-problems',
        linkText: 'View Project',
        tags: ['C#', 'Unity'],
    },
    {
        id: 'the-genie-of-light',
        title: '72 Hour Game Jam: The Genie of Light',
        date: 'April 2020',
        description: `2 friends and I participated in an online game jam where we made a game in 72 hours based on a theme.
        The theme was announced as Keep it alive. Sam and I designed and programmed the game and Ben created the music.
        The goal of the game is to traverse cave levels avoiding traps while keeping alive the light in the genie.
        The player can control the brightness of the light however the brighter the light the faster it will deplete if the light depletes completely the game becomes nearly impossible and so the player must optimise their usage of the light to get through the levels.
        We used Unity game engine and made 10 levels of increasing difficulty.
        After the jam the participants voted on each other's games and we came 1000th out of 5000 entries.
        In the end this was really good practice for completing a task over many days under time pressure in a team.`,
        alt: '',
        image: GenieOfLight,
        link: 'https://ldjam.com/events/ludum-dare/46/the-genie-of-light',
        linkText: 'View Project',
        tags: ['C#', 'Unity'],
    },
    {
        id: 'personal-website',
        title: 'Personal Website',
        date: 'September 2019',
        description: `This website is built using React and SCSS.`,
        github: 'https://github.com/RuairidhWilliamson/PersonalWebsite',
        alt: 'This Website',
        tags: ['React', 'Web'],
    },
    {
        id: 'rust-chess',
        title: 'Rust Chess Engine',
        date: 'September 2019',
        description: `A min max chess engine written in rust.
        This engine is similar to the python chess engine except rewritten in rust and it uses the python program to communicate with the lichess API.
        Min max is a great algorithm for turn based games like chess but it takes exponentially long for the number of turns to look ahead.
        This can be improved by removing some moves to look at because they are too bad but this can make it impossible to spot long sacrifices that benefit.
        I originally implemented a similar algorithm in python and tested it using the Lichess API to play against it and test it.
        Python is quite slow for this and so I turned to Rust to offer greater performance.
        It was more challenging to write it in Rust since I have never programmed using it before.
        The result is not perfect and is not very good at chess.`,
        alt: '',
        github: 'https://github.com/RuairidhWilliamson/chess',
        tags: ['Rust'],
    },
    {
        id: 'python-chess',
        title: 'Python Chess Engine',
        date: 'June 2019',
        description: `Min max algorithm implemented in python with lichess API.
        The program waits for a challenge from a player on Lichess and then plays using a Min Max algorithm looking 3 moves ahead which takes about 30 seconds to compute.
        Python is not an ideal language for doing large computations.`,
        alt: '',
        tags: ['Python'],
    },
    {
        id: 'car',
        title: 'Car Obstacle Avoidance',
        date: 'May 2019',
        description: `Simple neural network controlling car based on sensors.
        The car has 7 sensors: four proximity sensors, 2 velocity sensors (forward speed and sideways speed) and the angle between the car direction and the next goal.
        There are 2 outputs: the acceleration of the car and the steering.`,
        image: Car,
        alt: 'Car on track',
        tags: ['C#', 'Unity', 'Neural Network'],
    },
    {
        id: 'news-quiz',
        title: 'Current Affairs Quiz',
        date: 'December 2017',
        description: `Using google news API I pulled recent articles and compared their titles to find common words.
        Then one of the titles is given to the user with a word missing and the user must guess the missing word.
        There are 5 categories World, Business, Technology, Entertainment and Sport.`,
        image: WHT,
        alt: 'Quiz Category Selection',
        tags: ['JavaScript', 'Python', 'Web'],
    },
    {
        id: 'cards',
        title: 'Card Game',
        date: 'September 2017',
        description: `I challenged myself to write a web card game programming in different games.
        The entire project runs from a Python server which served static HTML, JavaScript, CSS and image files and also hosts websockets connections.
        The project includes a room system where users can create private rooms and invite others. Another part of the project was adding bots to play as well.
        I came up with different algorithms for the bots and allowed users to choose them.`,
        tags: ['JavaScript', 'Python', 'WebSockets', 'Web'],
    },
    {
        id: 'planet',
        title: 'Planet Simulation',
        date: 'August 2017',
        description: `Newtonian gravitational field simulation in web browser.
        This was written in canvas using javascript.
        It features tools to add bodies and experiment with solar wind.
        It also comes with existing scenarios and arrows to show velocity and acceleration.`,
        image: Planet,
        alt: 'Planet Sim Preview',
        content: <ResponsiveEmbed><Iframe src={'/planetSim/'} title='planetSim'/></ResponsiveEmbed>,
        tags: ['JQuery', 'Physics', 'JavaScript', 'Web'],
    },
].map(proj => ({
    ...proj,
    search: `${proj.id} ${proj.title} ${proj.tags} ${proj.description}`,
}));

export default projects;