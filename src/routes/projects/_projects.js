import WarpDriver from '../../projects/warp-driver.md';
import WarpDriverImg from '../../assets/images/warpdriver.png';

import PseudoHapticDrift from '../../projects/pseudo-haptic-drift.md';
import VRExperiment from '../../assets/images/vr_experiment.png';

import TimeTerrorists from '../../projects/time-terrorists.md';
import TimeTerroristsImg from '../../assets/images/time_terrorists.png';

import VideoLabeller from '../../projects/video-labeller.md';

import LSystems from '../../projects/l-systems.md';
import Fractal from '../../assets/images/fractal.PNG';

import TrolleyFolly from '../../projects/trolley-folly.md';
import TrolleyFollyImg from '../../assets/images/trolleyfolly.png';

import GenieOfLight from '../../projects/genie-of-light.md';
import GenieOfLightImg from '../../assets/images/genieoflight.png';

import PersonalWebsite from '../../projects/personal-website.md';

import RustChessEngine from '../../projects/rust-chess.md';

import PythonChessEngine from '../../projects/python-chess.md';

import Car from '../../projects/car.md';
import CarImg from '../../assets/images/car.png';

import NewsQuiz from '../../projects/news-quiz.md';
import Wht from '../../assets/images/wht.png';

import Cards from '../../projects/cards.md';

import Planet from '../../assets/images/planet.png';

const projects = [
    {
        title: 'Warp Driver',
        slug: 'warp-driver',
        date: 'October 2021',
        html: WarpDriver,
        image: WarpDriverImg,
        tags: ['C#', 'Unity']
    },
    {
        title: 'Pseudo Haptic Drift Correction in VR',
        slug: 'pseudo-haptic-drift-correction',
        date: 'Sept 2020 - May 2021',
        html: PseudoHapticDrift,
        image: VRExperiment,
        tags: ['VR', 'C#', 'Unity']
    },
    {
        title: 'The Time Terrorists',
        slug: 'time-terrorists',
        date: 'October 2020',
        html: TimeTerrorists,
        image: TimeTerroristsImg,
        tags: ['C#', 'Unity']
    },
    {
        slug: 'video-labeller',
        title: 'Video Labeller',
        date: 'July 2020',
        html: VideoLabeller,
        tags: ['C#'],
    },
    {
        slug: 'l-systems',
        title: 'L-Systems',
        date: 'July 2020',
        html: LSystems,
        image: Fractal,
        tags: ['Python'],
    },
    {
        slug: 'trolley-folly',
        title: '48 Hour Game Jam: Trolley Folly',
        date: 'July 2020',
        html: TrolleyFolly,
        image: TrolleyFollyImg,
        tags: ['C#', 'Unity'],
    },
    {
        slug: 'the-genie-of-light',
        title: '72 Hour Game Jam: The Genie of Light',
        date: 'April 2020',
        html: GenieOfLight,
        image: GenieOfLightImg,
        tags: ['C#', 'Unity'],
    },
    {
        slug: 'personal-website',
        title: 'Personal Website',
        date: 'September 2019',
        html: PersonalWebsite,
        image_alt: 'This Website',
        tags: ['React', 'Web'],
    },
    {
        slug: 'rust-chess',
        title: 'Rust Chess Engine',
        date: 'September 2019',
        html: RustChessEngine,
        tags: ['Rust'],
    },
    {
        slug: 'python-chess',
        title: 'Python Chess Engine',
        date: 'June 2019',
        html: PythonChessEngine,
        tags: ['Python'],
    },
    {
        slug: 'car',
        title: 'Car Obstacle Avoidance',
        date: 'May 2019',
        html: Car,
        image: CarImg,
        image_alt: 'Car on track',
        tags: ['C#', 'Unity', 'Neural Network'],
    },
    {
        slug: 'news-quiz',
        title: 'Current Affairs Quiz',
        date: 'December 2017',
        html: NewsQuiz,
        image: Wht,
        image_alt: 'Quiz Category Selection',
        tags: ['JavaScript', 'Python', 'Web'],
    },
    {
        slug: 'cards',
        title: 'Card Game',
        date: 'September 2017',
        html: Cards,
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

export default projects;
