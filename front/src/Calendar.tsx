import { useState } from 'react';
import { Calendar as RSCalendar } from 'rsuite';
import 'rsuite/Calendar/styles/index.css';
import { Settings, getAvailabilityForDay } from './App';

type CalendarProps = {
    getSettings: Settings;
};

function Calendar({ getSettings }: CalendarProps) {
  const [dates, setDates] = useState(new Map());

  function renderCell(date: Date): React.ReactNode {
    if (dates.has(date.getTime())) {
      return <><s>Booked</s></>
    }
    return null
  }

  function onSelect(date: Date) {
    const time = date.getTime();
    console.log(time);
    console.log(dates);
    if (dates.has(time)) {
      dates.delete(time);
    } else {
        const data = getAvailabilityForDay(getSettings.libraryDataSource, date);
        dates.set(time, data);
    }
    const new_map = new Map(dates);
    setDates(new_map);
  }

  return (
    <RSCalendar renderCell = {renderCell} onSelect = {onSelect} />
  )
}

export default Calendar
