#!/bin/bash

echo "🧪 Safe EXIF Testing Script"
echo "=========================="
echo ""
echo "This script safely tests EXIF support without breaking your working code."
echo ""

# Create backup
echo "📋 Step 1: Creating backup..."
cp Cargo.toml Cargo.toml.backup
cp src/media/processing/images.rs src/media/processing/images.rs.backup

echo "✅ Backup created"
echo ""

# Function to restore backup
restore_backup() {
    echo "🔄 Restoring backup..."
    mv Cargo.toml.backup Cargo.toml
    mv src/media/processing/images.rs.backup src/media/processing/images.rs
    echo "✅ Backup restored - your working code is safe!"
}

# Try different EXIF crate versions
echo "📋 Step 2: Testing EXIF dependency versions..."

# Test kamadak-exif versions
for version in "0.5" "0.4" "0.3" "0.2" "0.1"; do
    echo ""
    echo "🔍 Testing kamadak-exif version $version..."
    
    # Add dependency
    sed -i '' "s/# kamadak-exif.*$/kamadak-exif = \"$version\"               # EXIF data extraction/" Cargo.toml
    
    # Test compilation
    if cargo check --no-default-features --quiet 2>/dev/null; then
        echo "✅ Version $version works! Testing with features..."
        
        if cargo check --features="image-processing" --quiet 2>/dev/null; then
            echo "🎉 SUCCESS: kamadak-exif $version is compatible!"
            echo ""
            echo "📋 Step 3: Testing EXIF code..."
            
            # Uncomment EXIF code (simplified test)
            if grep -q "TODO: Re-enable when kamadak-exif" src/media/processing/images.rs; then
                echo "EXIF code is still commented out - would need manual uncommenting"
            fi
            
            echo ""
            echo "🎯 RESULT: EXIF dependency version $version works!"
            echo ""
            echo "Next steps to fully enable EXIF:"
            echo "1. Uncomment the EXIF code in src/media/processing/images.rs"
            echo "2. Change '_image_data' parameter back to 'image_data'"
            echo "3. Test compilation again"
            echo ""
            read -p "Do you want to keep this EXIF version enabled? (y/N): " -n 1 -r
            echo ""
            
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                echo "✅ Keeping EXIF enabled with version $version"
                rm -f Cargo.toml.backup src/media/processing/images.rs.backup
                echo ""
                echo "⚠️  Remember to:"
                echo "1. Uncomment EXIF code in src/media/processing/images.rs"
                echo "2. Test thoroughly before proceeding"
                exit 0
            else
                restore_backup
                echo "👍 EXIF disabled - your working code is preserved"
                exit 0
            fi
        else
            echo "❌ Version $version fails with image-processing feature"
        fi
    else
        echo "❌ Version $version doesn't work"
    fi
    
    # Reset for next test
    sed -i '' "s/kamadak-exif.*$/# kamadak-exif = \"0.3\"               # EXIF data extraction (temporarily disabled due to version issues)/" Cargo.toml
done

echo ""
echo "❌ No working EXIF version found"
restore_backup
echo ""
echo "💡 Recommendation: Keep EXIF disabled and focus on core messaging features"
echo "   You can always try again later when dependency versions are updated"
