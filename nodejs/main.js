console.time('nodeStatic:');
const airQuality = require('../json_files/airQuality.json');

const straightData = (data) => {

    const countyName = 'Union';
    const measureId = '292';
    const reportYear = '2011';

    return data
    .map(col => col
        .filter((item, index) => index > 7)
    )
    .map(item => ({
        measure_id: item[0],
        measure_name: item[1],
        measure_type: item[2],
        stratification_level: item[3],
        state_fips: item[4],
        state_name: item[5],
        county_fips: item[6],
        county_name: item[7],
        report_year: item[8],
        value: item[9],
        unit: item[10],
        unit_name: item[11],
        data_origin: item[12],
        monitor_only: item[13],
    }))
    // .filter(item => item.county_name === countyName)
    .filter(item => item.measure_id === measureId)
    .filter(item => item.report_year === reportYear);

};

const dataArray = straightData(airQuality.data);
console.timeEnd('nodeStatic:');
console.log(dataArray.length);

// console.log(dataArray[0].measure_id);