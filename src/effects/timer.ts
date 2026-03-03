import { useEffect, useState } from "react";

export function useFiveMinuteClock() {
    const [now, setNow] = useState(() => new Date());

    useEffect(() => {
        let intervalId: ReturnType<typeof setInterval> | undefined;

        const update = () => setNow(new Date());

        const scheduleNextInterval = () => {
            update();
            intervalId = setInterval(update, 5 * 60 * 1000);
        };

        const current = new Date();
        const msUntilNextFive =
            5 * 60 * 1000 -
            ((current.getMinutes() % 5) * 60 * 1000 +
                current.getSeconds() * 1000 +
                current.getMilliseconds());

        const timeoutId = setTimeout(scheduleNextInterval, msUntilNextFive);

        return () => {
            clearTimeout(timeoutId);
            if (intervalId) clearInterval(intervalId);
        };
    }, []);

    return now;
}