def print_table(initial_prod, initial_sales):
    months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]

    # Header for the table
    print(f"{'Month':<8} {'Production':<15} {'Sales':<15}")

    # Prints each month and the production and sales for that month
    for month in months:
        monthly_prod = initial_prod // 10
        monthly_sales = round(initial_sales * 0.07)

        if month == "Feb" or month == "Jun":
            monthly_prod = 0

        if month == "Sep" or month == "Dec":
            monthly_sales = round(initial_sales * 0.15)
    
        print(f"{month:<8} {monthly_prod:<15} {monthly_sales:<15}")

    # Total production and sales for the year
    print(f"{'Total':<8} {initial_prod:<15} {initial_sales:<15}")

def main():
    print_table(1000, 500)

if __name__ == "__main__":
    main()
