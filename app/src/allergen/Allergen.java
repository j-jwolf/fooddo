package allergen;

import java.util.*;

public class Allergen
{
	private Set<String> allergenTags; // set containing allergens in this instance
	private int id; // id of this instance
	/**
	* default constructor
	*/
	public Allergen()
	{
		allergenTags = new HashSet<String>();
		id = -1;
	}
	/**
	* constructor
	* @param final int id of this instance
	*/
	public Allergen(final int id)
	{
		allergenTags = new HashSet<String>();
		this.id = id;
	}
	/**
	* constructor
	* @param final int id of this instance
	* @param String[] list of allergens to add
	*/
	public Allergen(final int id, String allergens[])
	{
		allergenTags = new HashSet<String>();
		for(String s : allergens) {s = s.toLowerCase();}
		allergenTags.addAll(Arrays.asList(allergens));
		this.id = id;
	}
	/**
	* constructor
	* @param String[] list of allergens to add
	*/
	public Allergen(String allergens[])
	{
		allergenTags = new HashSet<String>();
		for(String s : allergens) {s = s.toLowerCase();}
		allergenTags.addAll(Arrays.asList(allergens));
	}
	/**
	* setter for id
	* @param final int id of this instance
	*/
	public void setId(final int id) {this.id = id;}
	/**
	* getter for id
	* @return int this instance's id
	*/
	public int getId() {return id;}
	/**
	* add allergen to tags
	* @param final String allergen to add
	*/
	public void addAllergen(final String allergen) {allergenTags.add(allergen.toLowerCase());}
	/**
	* add allergens to tags -- multiple
	* @param String[] allergens to add
	*/
	public void addAllergens(String allergens[])
	{
		for(String s : allergens) {s = s.toLowerCase();}
		allergenTags.addAll(Arrays.asList(allergens));
	}
	/**
	* remove allergen from tags
	* @param final String allergen to remove
	*/
	public void removeAllergen(final String allergen) {allergenTags.remove(allergen.toLowerCase());}
	/**
	* remove allergens from tags -- multiple
	* @param String[] allergens to remove
	*/
	public void removeAllergens(String allergens[])
	{
		for(String s : allergens) {s = s.toLowerCase();}
		allergenTags.removeAll(Arrays.asList(allergens));
	}
	/**
	* checks if allergen is contained in tags
	* @param final String allergen to check if contained
	* @return boolean true if found else false
	*/
	public boolean containsAllergen(final String allergen) {return allergenTags.contains(allergen.toLowerCase());}
	/**
	* checks if allergens are contained in tags -- multiple ALL MUST BE PRESENT FOR TRUE
	* @params String[] allergens to see if contained
	* @return boolean true if ALL items are in tags else false
	*/
	public boolean containsAllergens(String allergens[])
	{
		for(String s : allergens) {s = s.toLowerCase();}
		return allergenTags.containsAll(Arrays.asList(allergens));
	}
	/**
	* prints all allergen tags this instance contains -- not deprecated, only use for debugging
	* @deprecated
	*/
	@Deprecated
	public void displayAllergens()
	{
		StringBuilder line = new StringBuilder("");
		for(int i = 0; i < 50; i++) {line.append("=");}
		System.out.println(line.toString());
		Iterator it = allergenTags.iterator();
		int count = 0;
		System.out.println(new StringBuilder("My id: ").append(Integer.toString(id)).toString());
		while(it.hasNext()) {System.out.println(new StringBuilder(Integer.toString(++count)).append(": ").append(it.next()).toString());}
		System.out.println(line.toString());
	}
}
