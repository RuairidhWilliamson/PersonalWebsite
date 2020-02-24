import React from 'react';

import Iframe from 'components/iframe/iframe';
import { ResponsiveEmbed } from 'react-bootstrap';

import Planet from 'assets/images/planet.png';
import Car from 'assets/images/car.png';
import WHT from 'assets/images/wht.png';

const projects = [
    {
        id: 'personal-website',
        title: 'Personal Website',
        date: 'September 2019',
        description: `This website is built using React and SCSS.
        `,
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
        alt: 'Online Chess Board',
        tags: ['Rust'],
    },
    {
        id: 'python-chess',
        title: 'Python Chess Engine',
        date: 'June 2019',
        description: `Min max algorithm implemented in python with lichess API.
        The program waits for a challenge from a player on Lichess and then plays using a Min Max algorithm looking 3 moves ahead which takes about 30 seconds to compute.
        Python is not an ideal language for doing large computations.`,
        alt: 'Online Chess Board',
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
].map(proj => ({...proj, search: `${proj.id} ${proj.title} ${proj.tags} ${proj.description}`}));

export default projects;