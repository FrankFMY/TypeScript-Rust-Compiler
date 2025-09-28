// Тест UTF-8 кодировки с кириллицей
interface ТестИнтерфейс {
	имя: string;
	возраст: number;
}

class ТестКласс {
	private данные: string;

	constructor(данные: string) {
		this.данные = данные;
	}

	получитьДанные(): string {
		return this.данные;
	}
}

// Union типы
type СтрокаИлиЧисло = string | number;

// Enum с кириллицей
enum Статус {
	АКТИВНЫЙ = 'активный',
	НЕАКТИВНЫЙ = 'неактивный',
}

export { ТестКласс, ТестИнтерфейс, Статус };
