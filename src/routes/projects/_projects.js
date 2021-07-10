import Planet from '../../assets/images/planet.png';
import Car from '../../assets/images/car.png';
import Wht from '../../assets/images/wht.png';
import GenieOfLight from '../../assets/images/genieoflight.png';
import TrolleyFolly from '../../assets/images/trolleyfolly.png';
import Fractal from '../../assets/images/fractal.png';
import VRExperiment from '../../assets/images/vr_experiment.png';
import VRExperiment2 from '../../assets/images/vr_experiment2.png';

const projects = [
    {
        title: 'Pseudo Haptic Drift Correction in VR',
        slug: 'pseudo-haptic-drift-correction',
        date: 'September 2020 - May 2021',
        html: `<p>As part of my degree, I did a project over 6 months. The abstract of my project is as follows.</p>
        <p>Virtual reality is about stimulating the senses to create the illusion that the user exists in a simulated world. This is commonly achieved through the visual and auditory senses. It is also possible to stimulate the users sense of touch using haptics. Haptics are not widely available so we looked at pseudo haptics which is the method of creating the illusion of haptic stimulation using other senses. A recently proposed pseudo haptic mass illusion discussed how to create the illusion of objects having mass without using haptics. This project is about refining this pseudo haptic mass illusion. The technique involves creating a drift between the physical and real controller positions / rotations. We look at different techniques to correct this pseudo haptic drift whilst maintaining the embodiment illusion. In the project we implemented the pseudo haptic mass technique in a virtual reality application. We designed and implemented 8 techniques to correct the pseudo haptic drift. We ran a user study to compare the most effective correction method to realign the pseudo haptic drift. We find that our results indicate some techniques are better at correcting the drift than others whilst maintaining the embodiment illusion.</p>
        <img src="${VRExperiment}" alt="VR Experiment">
        <img src="${VRExperiment2}" alt="VR Experiment Difficulty Question">
        `,
        image: VRExperiment,
        tags: ['VR', 'C#', 'Unity']
    },
    {
        slug: 'video-labeller',
        title: 'Video Labeller',
        date: 'July 2020',
        html: `<p>My grandfather had a cine film camera and his films were digitised however they are not in order.</p>
        <p>I wrote a program that takes videos divided into scenes and allows the user to label and tag the videos.</p>
        <p>The program also allows the user to search for tags and play the selected videos in sequence.</p>
        <p>This is useful because relatives often only want to see videos that are relevant to them and sorting through 14 hours of video takes too long.</p>
        <p>The program uses FFmpeg to convert the videos into short clips.</p>`,
        tags: ['C#'],
    },
    {
        slug: 'l-systems',
        title: 'L-Systems',
        date: 'July 2020',
        html: `<p>I was interested in L-systems which is a way of describing fractal patterns.</p>
        <p>It works by having a start string e.g. <code>"X"</code>.</p>
        <p>Then having rules such as <code>X -> XAX</code> and <code>A -> AB</code> would result in the following:</p>
        <pre><code>
        n = 0: X
        n = 1: XAX
        n = 2: XAXABXAX
        n = 3: XAXABXAXABBXAXABXAX
        n = 4: XAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAX
        n = 5: XAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAXABBBBXAXABXAXABBXAXABXAXABBBXAXABXAXABBXAXABXAX
        ...
        </code></pre>
        <p>Then by creating rules based on the string for example forward, turn left and turn right we can draw images.</p>
        <a href=https://en.wikipedia.org/wiki/L-system>L-Systems Wikipedia</a>`,
        image: Fractal,
        tags: ['Python'],
    },
    {
        slug: 'trolley-folly',
        title: '48 Hour Game Jam: Trolley Folly',
        date: 'July 2020',
        html: `
        <p>A friend and I participated in an online game jam where we made a game in 48 hours based on a theme.</p>
        <p>The theme was announced as Out of Control. Sam and I created a game where the player is trying to control and out of control shopping trolley as you speed around a supermarket collecting as many items as possible.</p>
        <p>The more items you stack in your cart the easier it is for items to fall off. The player has to balance the items, avoid crashing into things and take the items to the checkout.</p>
        <a href=https://wilkoco.itch.io/trolley-problems target=_blank>View Project</a>
        `,
        image: TrolleyFolly,
        tags: ['C#', 'Unity'],
    },
    {
        slug: 'the-genie-of-light',
        title: '72 Hour Game Jam: The Genie of Light',
        date: 'April 2020',
        html: `
        <p>2 friends and I participated in an online game jam where we made a game in 72 hours based on a theme.</p>
        <p>The theme was announced as Keep it alive. Sam and I designed and programmed the game and Ben created the music.</p>
        <p>The goal of the game is to traverse cave levels avoiding traps while keeping alive the light in the genie.</p>
        <p>The player can control the brightness of the light however the brighter the light the faster it will deplete if the light depletes completely the game becomes nearly impossible and so the player must optimise their usage of the light to get through the levels.</p>
        <p>We used Unity game engine and made 10 levels of increasing difficulty.</p>
        <p>After the jam the participants voted on each other's games and we came 1000th out of 5000 entries.</p>
        <p>In the end this was really good practice for completing a task over many days under time pressure in a team.</p>
        <a href=https://ldjam.com/events/ludum-dare/46/the-genie-of-light target=_blank>View Project</a>
        `,
        image_alt: '',
        image: GenieOfLight,
        tags: ['C#', 'Unity'],
    },
    {
        slug: 'personal-website',
        title: 'Personal Website',
        date: 'September 2019',
        html: `
        <p>This website was built using React and SCSS.</p>
        <p>In July 2021 it was updated to use svelte and sapper instead.</p>
        <a href=https://github.com/RuairidhWilliamson/PersonalWebsite target=_blank>View on GitHub</a>
        `,
        image_alt: 'This Website',
        tags: ['React', 'Web'],
    },
    {
        slug: 'rust-chess',
        title: 'Rust Chess Engine',
        date: 'September 2019',
        html: `
        <p>A min max chess engine written in rust.</p>
        <p>This engine is similar to the python chess engine except rewritten in rust and it uses the python program to communicate with the lichess API.</p>
        <p>Min max is a great algorithm for turn based games like chess but it takes exponentially long for the number of turns to look ahead.</p>
        <p>This can be improved by removing some moves to look at because they are too bad but this can make it impossible to spot long sacrifices that benefit.</p>
        <p>I originally implemented a similar algorithm in python and tested it using the Lichess API to play against it and test it.</p>
        <p>Python is quite slow for this and so I turned to Rust to offer greater performance.</p>
        <p>It was more challenging to write it in Rust since I have never programmed using it before.</p>
        <p>The result is not perfect and is not very good at chess.</p>
        <a href=https://github.com/RuairidhWilliamson/chess target=_blank>View on GitHub</a>
        `,
        image_alt: '',
        tags: ['Rust'],
    },
    {
        slug: 'python-chess',
        title: 'Python Chess Engine',
        date: 'June 2019',
        html: `
        <p>Min max algorithm implemented in python with lichess API.</p>
        <p>The program waits for a challenge from a player on Lichess and then plays using a Min Max algorithm looking 3 moves ahead which takes about 30 seconds to compute.</p>
        <p>Python is not an ideal language for doing large computations.</p>
        `,
        image_alt: '',
        tags: ['Python'],
    },
    {
        slug: 'car',
        title: 'Car Obstacle Avoidance',
        date: 'May 2019',
        html: `
        <p>Simple neural network controlling car based on sensors.</p>
        <p>The car has 7 sensors: four proximity sensors, 2 velocity sensors (forward speed and sideways speed) and the angle between the car direction and the next goal.</p>
        <p>There are 2 outputs: the acceleration of the car and the steering.</p>
        `,
        image: Car,
        image_alt: 'Car on track',
        tags: ['C#', 'Unity', 'Neural Network'],
    },
    {
        slug: 'news-quiz',
        title: 'Current Affairs Quiz',
        date: 'December 2017',
        html: `
        <p>Using google news API I pulled recent articles and compared their titles to find common words.</p>
        <p>Then one of the titles is given to the user with a word missing and the user must guess the missing word.</p>
        <p>There are 5 categories World, Business, Technology, Entertainment and Sport.</p>
        `,
        image: Wht,
        image_alt: 'Quiz Category Selection',
        tags: ['JavaScript', 'Python', 'Web'],
    },
    {
        slug: 'cards',
        title: 'Card Game',
        date: 'September 2017',
        html: `
        <p>I challenged myself to write a web card game programming in different games.</p>
        <p>The entire project runs from a Python server which served static HTML, JavaScript, CSS and image files and also hosts websockets connections.</p>
        <p>The project includes a room system where users can create private rooms and invite others. Another part of the project was adding bots to play as well.</p>
        <p>I came up with different algorithms for the bots and allowed users to choose them.</p>
        `,
        tags: ['JavaScript', 'Python', 'WebSockets', 'Web'],
    },
    {
        slug: 'planet',
        title: 'Planet Simulation',
        date: 'August 2017',
        html: `
        <p>Newtonian gravitational field simulation in web browser.</p>
        <p>This was written in HTML canvas using javascript.</p>
        <p>It features tools to add bodies and experiment with solar wind.</p>
        <p>It also comes with existing scenarios and arrows to show velocity and acceleration.</p>
        <iframe src="/planetSim/" title="planetSim" frameborder="0"/>
        `,
        image: Planet,
        image_alt: 'Planet Sim Preview',
        tags: ['JQuery', 'Physics', 'JavaScript', 'Web'],
    },
];

projects.forEach(project => {
    project.html = project.html.replace(/^\t{3}/gm, '');
});

export default projects;
