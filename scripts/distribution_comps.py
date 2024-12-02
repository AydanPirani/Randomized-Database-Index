import os
import pandas as pd

def process_csv_files(input_directory, output_file):
    # Initialize an empty list to store all processed data
    all_processed_data = []

    # Iterate through all CSV files in the input directory
    for filename in os.listdir(input_directory):
        if filename.endswith('.csv') and '10000ops' in filename:
            file_path = os.path.join(input_directory, filename)
            # Read the CSV file
            df = pd.read_csv(file_path)
            # Add a column for the filename (workload)
            df['workload'] = filename.split('-')[0]

            # Group by 'index' and calculate the required statistics
            grouped_data = df.groupby(['index']).agg({
                'time': ['min', 'mean', 'max']
            }).reset_index()

            # Flatten the MultiIndex columns
            grouped_data.columns = ['index', 'min', 'mean', 'max']

            # Calculate quantiles separately
            quantiles = df.groupby(['index'])['time'].quantile([0.25, 0.5, 0.75]).unstack(level=-1)
            quantiles.columns = ['25%', '50%', '75%']
            quantiles = quantiles.reset_index()

            # Merge the quantiles with the grouped data
            grouped_data = pd.merge(grouped_data, quantiles, on='index')

            # Add the workload column back to the grouped data
            grouped_data['workload'] = filename.split('-')[0]

            # Append the processed data to the list
            all_processed_data.append(grouped_data)

    # Concatenate all processed data into a single DataFrame
    final_data = pd.concat(all_processed_data, ignore_index=True)

    # Sort the data by 'index' and then by 'workload'
    final_data = final_data.sort_values(by=['index', 'workload'])

    # 3 decimal places
    final_data[['min', 'mean', 'max', '25%', '50%', '75%']] = final_data[['min', 'mean', 'max', '25%', '50%', '75%']].round(3)

    # Write the final data to the output file
    final_data.to_csv(output_file, index=False)

# Example usage
input_directory = 'out/generated/'  # Ensure this is a directory
output_file = 'dog.csv'
process_csv_files(input_directory, output_file)