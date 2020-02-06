import React from 'react';
import PropTypes from 'prop-types';

import Icon from 'components/materialIcon/materialIcon';

import './search.scss';

export default class Search extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    handleChange(ev) {
        this.props.onChange && this.props.onChange(ev.target.value);
    }

    handleClear() {
        this.props.onChange && this.props.onChange('');
        this.setState({ value: '' });
    }

    render() {
        return <div className='search-container'>
            <div className='search'>
                <Icon icon='search'/>
                <input
                    type='text'
                    value={this.props.value}
                    onChange={this.handleChange.bind(this)}
                    autoFocus={this.props.autoFocus}
                    maxLength={this.props.maxLength}
                    placeholder={this.props.placeholder}
                />
                <Icon className={`clear ${this.props.value && 'visible'}`} onClick={this.handleClear.bind(this)} icon='clear'/>
            </div>
        </div>;
    }
}

Search.propTypes = {
    placeholder: PropTypes.string,
    autoFocus: PropTypes.bool,
    maxLength: PropTypes.number,
    onChange: PropTypes.func,
    value: PropTypes.string,
};

Search.defaultProps = {
    placeholder: 'Search...',
    autoFocus: false,
    maxLength: 160,
    value: '',
};
