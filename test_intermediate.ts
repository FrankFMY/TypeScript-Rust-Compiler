// Intermediate TypeScript test
interface Car {
	brand: string;
	model: string;
	year: number;
}

class CarService {
	private cars: Car[] = [];

	addCar(car: Car): void {
		this.cars.push(car);
	}

	getCars(): Car[] {
		return this.cars;
	}
}

function createCar(brand: string, model: string, year: number): Car {
	return { brand, model, year };
}

const service = new CarService();
const car = createCar('Toyota', 'Camry', 2020);
service.addCar(car);

console.log('Car added:', car.brand, car.model);
