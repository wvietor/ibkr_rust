// === instrumentList === //

xmllint --xpath '//ScanParameterResponse/InstrumentList[@varName="instrumentList"]/Instrument' ./resp_scan_param_rust.xml

// === fullInstrumentList === //

xmllint --xpath '//ScanParameterResponse/InstrumentList[@varName="fullInstrumentList"]/Instrument' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/InstrumentList[@varName="fullInstrumentList"]/Instrument/name | //ScanParameterResponse/InstrumentList[@varName="fullInstrumentList"]/Instrument/type' ./resp_scan_param_rust.xml

// === locationTree === //

xmllint --xpath '//ScanParameterResponse/LocationTree[@varName="locationTree"]/Location' ./resp_scan_param_rust.xml

// === ScanType === //

xmllint --xpath '//ScanParameterResponse/ScanTypeList[@varName="scanTypeList"]/ScanType/scanCode/text()' ./resp_scan_param_rust.xml
xmllint --xpath '//ScanParameterResponse/ScanTypeList[@varName="scanTypeList"]/ScanType/displayName/text()' ./resp_scan_param_rust.xml
xmllint --xpath '//ScanParameterResponse/ScanTypeList[@varName="scanTypeList"]/ScanType/instruments/text()' ./resp_scan_param_rust.xml

// uniq instruments in ScanType
tr ',' '\n' < scan_codes2 | sort | uniq

xmllint --xpath '//ScanParameterResponse/ScanTypeList[@varName="scanTypeList"]/ScanType/scanCode/text() | //ScanParameterResponse/ScanTypeList[@varName="scanTypeList"]/ScanType/displayName/text() | //ScanParameterResponse/ScanTypeList[@varName="scanTypeList"]/ScanType/instruments/text() ' ./resp_scan_param_rust.xml

// === settingList === //

xmllint --xpath '//ScanParameterResponse/SettingList[@varName="settingList"]' ./resp_scan_param_rust.xml

// === filterList === //

xmllint --xpath '//ScanParameterResponse/FilterList[@varName="filterList"]' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/FilterList[@varName="filterList"]/RangeFilter/id/text()' ./resp_scan_param_rust.xml
xmllint --xpath '//ScanParameterResponse/FilterList[@varName="filterList"]/SimpleFilter/id/text()' ./resp_scan_param_rust.xml
xmllint --xpath '//ScanParameterResponse/FilterList[@varName="filterList"]/TripleComboFilter/id/text()' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/FilterList[@varName="filterList"]/RangeFilter/id/text() | //ScanParameterResponse/FilterList[@varName="filterList"]/SimpleFilter/id/text() | //ScanParameterResponse/FilterList[@varName="filterList"]/TripleComboFilter/id/text()' ./resp_scan_param_rust.xml

// === scannerLayoutList === //

xmllint --xpath '//ScanParameterResponse/ScannerLayoutList[@varName="scannerLayoutList"]/ScannerLayout' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/InstrumentGroupList[@varName="instrumentGroupList"]/InstrumentGroup' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/InstrumentGroupList[@varName="instrumentGroupList"]/InstrumentGroup' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/SimilarProductsDefaults[@varName="similarProductsDefaults"]' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/MainScreenDefaultTickers[@varName="mainScreenDefaultTickers"]' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/MainScreenDefaultTickers[@varName="mainScreenDefaultTickers"]' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/ColumnSets[@varName="columnSets"]' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/SidecarScannerDefaults[@varName="sidecarScannerDefaults"]' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/SidecarScannerTemplateList[@varName="scannerTemplateList"]/SidecarScannerTemplate' ./resp_scan_param_rust.xml

xmllint --xpath '//ScanParameterResponse/ScannerProductTypeList[@varName="scannerProductTypeList"]' ./resp_scan_param_rust.xml

//-
<FieldsConfigurationList varName="fieldsConfigurationList">
<AdvancedScannerDefaults varName="advancedScannerDefaults">

//+ AbstractField and SimpleFilter
xmllint --xpath '//ScanParameterResponse/FilterList[@varName="uiFilters"]' ./resp_scan_param_rust.xml
xmllint --xpath '//ScanParameterResponse/FilterList[@varName="uiFilters"]/SimpleFilter' ./resp_scan_param_rust.xml
