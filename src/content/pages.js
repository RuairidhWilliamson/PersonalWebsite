import Homepage from 'containers/homepage/homepage';
import Projects from 'containers/projects/projects';
import Contact from 'containers/contact/contact';
import HexQuizContainer from 'containers/hexQuiz/hexQuiz';

const pages = [
    {
        path: '/',
        exact: true,
        label: 'RW',
        component: Homepage,
        material_icon: 'home',
    },
    {
        path: '/projects',
        exact: true,
        label: 'Projects',
        component: Projects,
        material_icon: 'list',
    },
    {
        path: '/hex-quiz',
        exact: true,
        label: 'Hex Quiz',
        component: HexQuizContainer,
        material_icon: 'code',
    },
    {
        path: '/contact',
        exact: true,
        label: 'Contact',
        component: Contact,
        material_icon: 'perm_contact_calendar',
    },
];

export default pages;