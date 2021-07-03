import Homepage from 'containers/homepage/homepage';
import Projects from 'containers/projects/projects';
import Contact from 'containers/contact/contact';

const pages = [
    {
        path: '/',
        exact: true,
        hide_navbar: true,
        label: 'Home',
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
        path: '/contact',
        exact: true,
        label: 'Contact',
        component: Contact,
        material_icon: 'perm_contact_calendar',
    },
];

export default pages;