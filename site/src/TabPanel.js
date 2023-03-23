function TabPanel({children, currentValue, selfValue}) {
    return (
        <div
            role="tabpanel" // ???
            hidden={currentValue !== selfValue}
            id={`simple-tabpanel-${selfValue}`}
        >
            {currentValue === selfValue && (
                <div>
                    {children}
                </div>
            )}
        </div>
    );
}

export default TabPanel;