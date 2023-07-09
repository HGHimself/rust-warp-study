
// range of [a, a + b-1]
const random = (a, b) => a + Math.floor(Math.random() * b);
const coinFlip = () => random(0, 2);

// f(t) = y
// given an amplitude, omega, and time(x) in a usual simple harmonic motion fashion,
// as well as an additional array of activated fourier elements and a sequence to sum them using the proper transformation
// you get back the y value at the given time t
const fourier = (amplitude, omega, time, arr, sequence) =>
    amplitude * sequence(omega, time, arr);

// for a square wave, b_n = 6/(n * Pi)
const squareWaveSin = (omega, time, beta) =>
    (2 / (beta * Math.PI)) * Math.sin(omega * time * beta);
const squareWaveCos = (omega, time, beta) =>
    (2 / (beta * Math.PI)) * Math.cos(omega * time * beta);

// the sequence for a square wave is 3/2 + odds.sum((n) => b_n * sin(n * t))
// odds does not have to be odd
const squareWaveSequenceSin = (omega, time, odds) =>
    odds.reduce((acc, beta) => acc + squareWaveSin(omega, time, beta), 0);
const squareWaveSequenceCos = (omega, time, odds) =>
    odds.reduce((acc, beta) => acc + squareWaveCos(omega, time, beta), 0);

const getRgbSpectrumArray = (i) => {
    const r = Math.round(127 * Math.cos(i + 2 * Math.PI)) + 128;
    const g = Math.round(127 * Math.sin(i + 2 * Math.PI)) + 128;
    const b = Math.round(127 * Math.cos(i + Math.PI)) + 128;

    return [r, g, b];
};

const getSpectrumPosition = (i, a) => {
    const [r, g, b] = getRgbSpectrumArray(i);
    return a ? `rgba(${r}, ${g}, ${b}, ${a})` : `rgb(${r}, ${g}, ${b})`;
};

class Background {
    constructor(props) {
        this.props = props;

        const { width, height } = props;
        this.width = width;
        this.height = height;

        this.props.amplitudeMultiplier = 1;
        this.calculateProps();

        const [background, gradient] = this.props.background;

        this.svg = d3.select("body")
            .insert("svg", ":first-child")
            .style("position", "absolute")
            .style("z-index", "-1")
            .style("background-color", background)
            .style("background-image", gradient)
            .attr("width", width)
            .attr("height", height);

        this.svg.append("defs");

        this.svg.append("g").attr("class", "lines");
        this.svg.append("g").attr("class", "rect");

        this.update();
    }

    resize(width, height) {
        const { svg } = this;

        this.width = width;
        this.height = height;

        svg.attr("width", this.width).attr("height", this.height);

        this.update();
    }

    setOptions(options) {
        this.props = options;
        this.calculateProps();
        this.update();
    }

    getDrawer(batch) {
        const {
            numbers,
            xAmplitude,
            yAmplitude,
            amplitudeMultiplier,
            period,
            offset,
            xMultiplier,
            yMultiplier,
        } = this.props;

        const squarewaveTransformX = (time) =>
            fourier(
                xAmplitude * amplitudeMultiplier,
                2 * Math.PI * period,
                time,
                numbers,
                squareWaveSequenceSin
            );
        const squarewaveTransformY = (time) =>
            fourier(
                yAmplitude * amplitudeMultiplier,
                2 * Math.PI * period,
                time,
                numbers,
                squareWaveSequenceCos
            );

        const arc = Array.from({ length: 4 }, (_, i) => [
            squarewaveTransformX(314 * xMultiplier * (i + batch + offset)),
            squarewaveTransformY(314 * yMultiplier * (i + batch + offset)),
        ]);

        return !this.props.curve ? d3.line().curve(d3.curveBasisOpen)(arc) : d3.line()(arc);
    }

    getColor(d) {
        const {
            props: { color, count },
        } = this;

        return !color
            ? "currentColor"
            : getSpectrumPosition(color + d / (count * 0.4));
    }

    calculateProps() {
        const { frequency, count } = this.props;
        this.props.amplitudeMultiplier = 1;
        this.props.period = frequency ? 1 / (frequency * 314.1) : frequency;
        this.props.data = Array.from({ length: count }, (_, i) => i);
    }

    update() {
        const {
            width,
            height,
            props: { thickness, data, background, gradient },
        } = this;

        this.svg
            .style("background-color", background)
            .style("background-image", gradient);

        this.svg
            .select("g.lines")
            .selectAll("path.door")
            .data(data)
            .join(
                (enter) =>
                    enter
                        .append("path")
                        .attr("fill", "none")
                        .attr("class", "door")
                        .attr("stroke", "currentColor")
                        .attr("stroke-width", thickness)
                        .attr("stroke", (d) => this.getColor(d))
                        .attr("transform", `translate(${width / 2},${height / 2})`)
                        .attr("d", (d) => this.getDrawer(d)),
                (update) =>
                    update
                        .attr("stroke-width", thickness)
                        .attr("stroke", (d) => this.getColor(d))
                        .attr("transform", `translate(${width / 2},${height / 2})`)
                        .attr("d", (d) => this.getDrawer(d))
            );

        !this.props.hideNoise && this.svg
            .select("defs")
            .selectAll("filter#noiseFilter")
            .data([0])
            .join(
                (enter) =>
                    enter
                        .append("filter")
                        .attr("id", "noiseFilter")
                        .attr("x", "0")
                        .attr("y", "0")
                        .attr("height", height)
                        .attr("width", width)
                        .append("feTurbulence")
                        .attr("type", "fractalNoise")
                        .attr("baseFrequency", this.props.noiseFreqOne)
                        .attr("numOctaves", this.props.numOctaves)
                        .attr("stitchTiles", "stitch"),
                (update) =>
                    update
                        .attr("height", height)
                        .attr("width", width)
                        .selectAll("feTurbulence")
                        .attr("baseFrequency", this.props.noiseFreqOne)
                        .attr("numOctaves", this.props.numOctaves),
                (exit) => exit
            );

        !this.props.hideNoise && this.svg
            .select("g.rect")
            .selectAll("rect")
            .data([0])
            .join(
                (enter) =>
                    enter
                        .append("rect")
                        .attr("class", "noise")
                        .attr("width", width)
                        .attr("height", height)
                        .attr("filter", "url(#noiseFilter)"),
                (update) => update.attr("height", height).attr("width", width)
            );

        this.props.showProps &&
            this.svg
                .selectAll("text.details")
                .data(
                    Object.keys(this.props)
                        .filter(key => !['data', 'gradient'].includes(key))
                        .map((key) => `${key}: ${this.props[key]}`)
                )
                .join(
                    (enter) =>
                        enter
                            .append("text")
                            .attr("class", "details")
                            .attr("x", this.props.width - 10)
                            .attr("y", (_, i) => 100 + 10 * (i + 1))
                            .attr("font-size", 12)
                            .attr("text-anchor", "end")
                            .text((d) => d),
                    (update) => update.text((d) => d),
                    (exit) => exit
                );
    }
}

const showBackground = ({
    count,
    frequency,
    xAmplitude,
    yAmplitude,
    xMultiplier,
    yMultiplier,
    color,
    thickness,
}) => {
    const props = {
        height: window.innerHeight,
        width: window.innerWidth,
        count: count ?? random(50, 100),
        offset: 0,
        frequency: frequency ?? random(1, 15),
        xAmplitude: xAmplitude ?? random(1000, 1500),
        yAmplitude: yAmplitude ?? random(600, 1000),
        xMultiplier: xMultiplier ?? random(1, 15),
        yMultiplier: yMultiplier ?? random(1, 15),
        color: color ?? random(3, 2000),
        thickness: thickness ?? random(50, 75),
        numbers: [1, 3, 5, 7],
        curve: 0,
        noiseFreqOne: 2.78,
        numOctaves: 6,
        hideNoise: 1,
        showProps: 1,
        background: "rgb(255, 153, 233)",
        gradient: "radial-gradient(at 32% 33%, rgb(252, 146, 194) 0px, transparent 50%), radial-gradient(at 72% 16%, rgb(249, 93, 106) 0px, transparent 50%), radial-gradient(at 26% 44%, rgb(95, 141, 227) 0px, transparent 50%), radial-gradient(at 74% 60%, rgb(56, 101, 250) 0px, transparent 50%), radial-gradient(at 18% 76%, rgb(239, 216, 123) 0px, transparent 50%), radial-gradient(at 89% 65%, rgb(234, 164, 72) 0px, transparent 50%), radial-gradient(at 65% 72%, rgb(165, 226, 116) 0px, transparent 50%)"
    }
    
    var background = new Background(props)
    
    
    window.addEventListener("resize", () => {
        background.resize(window.innerWidth, window.innerHeight)
    });
    
    let lastKnownScrollPosition = 0;
    let ticking = false;
    const content = document.getElementById("content")
    content.addEventListener("scroll", (event) => {
        lastKnownScrollPosition = content.scrollTop;
    
        if (!ticking) {
            window.requestAnimationFrame(() => {
                background.setOptions({
                    ...props,
                    offset: props.offset + (lastKnownScrollPosition / 10000)
                });
                ticking = false;
            });
    
            ticking = true;
        }
    });
}