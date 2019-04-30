console.time('nodeStatic:');
const airQuality = require('../json_files/airQuality.json');

const reducer = (data) => {
    const redux = (accumulator, currentValue) => accumulator + currentValue.value;
    const sum_value = data.reduce(redux, 0);
    return sum_value / data.length;
}

const straightData = (data) => {

    // const countyName = 'El Paso';
    // const reportYear = 2012;
    const measureId = 292;

    const myData = data
    .map(col => col
        .filter((item, index) => index > 7)
    )
    .map(item => ({
        measure_id: parseInt(item[0]),
        measure_name: item[1],
        measure_type: item[2],
        stratification_level: item[3],
        state_fips: parseInt(item[4]),
        state_name: item[5],
        county_fips: parseInt(item[6]),
        county_name: item[7],
        report_year: parseInt(item[8]),
        value: parseFloat(item[9]),
        unit: item[10],
        unit_name: item[11],
        data_origin: item[12],
        monitor_only: parseInt(item[13]),
    }))
    // .filter(item => item.value > 1 && item.measure_id === measureId)
    .filter(item => item.measure_id === measureId)

    console.log(`Average: ${reducer(myData)}`);

    return myData;
};

const dataArray = straightData(airQuality.data);
console.timeEnd('nodeStatic:');
console.log(dataArray.length);

// console.log(dataArray[0].measure_id);