const Component = () => {
    return (
        <div>
            <div />
        </div>
    );
}

const Component2 = () => {
    return (
        <div>
            <div />
            ;
        </div>
    );
}

const Component3 = () => {
    return (
        <div>
          <div />{';'}
        </div>
    );
}

const Component4 = () => {
    return (
        <div>
          <div />&#59;
        </div>
    );
}

const Component5 = () => {
    return (
        <div>
          <span>;</span>
          <span />;<span />
          text; text;
          &amp;
        </div>
    );
}

const Component6 = () => {
    return <div />;
}

const Component7 = () => {
    return (
        <div>
            <div />text;
        </div>
    );
}
