console.time('Node:');
const airQuality = require('../json_files/airQuality.json');

const reducer = data => {
  const redux = (accumulator, currentValue) => accumulator + currentValue.value;
  const sum_value = data.reduce(redux, 0);
  return sum_value / data.length;
}

const createDataObject = data => {

  const myData = data
    .map(item => ({
      measure_id: parseInt(item[8]),
      measure_name: item[9],
      measure_type: item[10],
      stratification_level: item[11],
      state_fips: parseInt(item[12]),
      state_name: item[13],
      county_fips: parseInt(item[14]),
      county_name: item[15],
      report_year: parseInt(item[16]),
      value: parseFloat(item[17]),
      unit: item[18],
      unit_name: item[19],
      data_origin: item[20],
      monitor_only: parseInt(item[21]),
    }))

  return myData;
};

const getMeasureTypes = data => {
  const myTypesArray = data
    .map(item => item.measure_type);

  return [...new Set(myTypesArray)];
};

const getMeasureIds = data => {
  const myArray = data
    .map(item => item.measure_id);

  return [...new Set(myArray)];
};

const filterByType = (data, type) => {
  return data
    .filter(item => item.measure_type === type);
};

const data = createDataObject(airQuality.data);
const myMeasureIds = getMeasureIds(data);
const myTypes = getMeasureTypes(data);

const counts = filterByType(data, 'Counts');
const avgCounts = reducer(counts);

const percent = filterByType(data, 'Percent');
const avgPct = reducer(percent);

const average = filterByType(data, 'Average');
const avgAvg = reducer(average);


console.timeEnd('Node:');

console.log(`measure_ids ${myMeasureIds}`);
console.log(`measure_types ${myTypes}`);
console.group();
console.log(` `);
console.log(`Total Counts registers ${counts.length}`);
console.log(`Average in Counts ${avgCounts}`);
console.log(` `);
console.groupEnd();
console.group();
console.log(` `);
console.log(`Total Percent registers ${percent.length}`);
console.log(`Average in Percent ${avgPct}`);
console.log(` `);
console.groupEnd();
console.group();
console.log(` `);
console.log(`Total Average registers ${average.length}`);
console.log(`Average in Average ${avgAvg}`);
console.log(` `);
console.groupEnd();
