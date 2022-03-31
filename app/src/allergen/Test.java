package allergen;

public class Test
{
	public static <T> void print(T t) {System.out.print(t);}
	public static <T> void println(T t) {System.out.println(t);}
	public static void main(final String args[])
	{
		String test1[] = {"milk", "eggs"}, test2[] = {"soy", "wheat", "dairy"};
		Allergen allergens[] = {
			new Allergen(),
			new Allergen(100),
			new Allergen(101, test2),
			new Allergen(test1)
		};
		String testSet[] = {"these", "are", "flags"};
		for(Allergen a : allergens)
		{
			a.displayAllergens();
			a.addAllergen("shellfish");
			a.displayAllergens();
			println(new StringBuilder("contains shellfish?: ").append(Boolean.toString(a.containsAllergen("shellfish"))).toString());
			a.removeAllergen("shellfish");
			a.displayAllergens();
			a.addAllergens(testSet);
			a.displayAllergens();
			println(new StringBuilder("Contains these,are,flags?: ").append(Boolean.toString(a.containsAllergens(testSet))).toString());
			a.removeAllergens(testSet);
			a.displayAllergens();
		}
		return;
	}
}
