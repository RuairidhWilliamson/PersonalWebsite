import React from 'react';
import PropTypes from 'prop-types';

import Pages from 'content/pages';
import Icon from 'components/materialIcon/materialIcon';
import { Navbar, Nav } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';

import './header.scss';

const NavItem = (props) => (
    <LinkContainer exact to={props.to}>
        <Nav.Link>
            <div className="nav-item"><Icon icon={props.materialIcon}/> <div>{props.label}</div></div>
        </Nav.Link>
    </LinkContainer>
);

NavItem.propTypes = {
    to: PropTypes.string,
    label: PropTypes.string,
    active: PropTypes.bool,
    materialIcon: PropTypes.string,
    indent: PropTypes.number,
};

export default class Header extends React.Component {

    renderNavItem(page, indent = 1) {
        return <NavItem
            key={page.label}
            active={this.props.location.pathname === page.path}
            materialIcon={page.material_icon}
            label={page.label}
            to={page.path}
            indent={indent}
        />;
    }

    render() {
        return <Navbar bg="dark" variant="dark">
            <LinkContainer to="/">
                <Navbar.Brand>RW</Navbar.Brand>
            </LinkContainer>
            <Nav className="mr-auto nav-bar">
                {Pages.map(page => this.renderNavItem(page))}
            </Nav>
        </Navbar>;
    }
}

Header.propTypes = {
    location: PropTypes.object,
};
