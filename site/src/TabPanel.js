function TabPanel({children, value, index}) {
    return (
        <div
            role="tabpanel" // ???
            hidden={value !== index}
            id={`simple-tabpanel-${index}`}
        >
            {value === index && (
                <div>
                    {children}
                </div>
            )}
        </div>
    );
}

export default TabPanel;